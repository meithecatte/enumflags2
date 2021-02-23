#![recursion_limit = "2048"]
extern crate proc_macro;
#[macro_use]
extern crate quote;

use std::convert::TryFrom;
use syn::{Ident, Item, ItemEnum, spanned::Spanned, parse_macro_input};
use proc_macro2::{TokenStream, Span};
use proc_macro::TokenTree;

struct Flag {
    name: Ident,
    span: Span,
    value: FlagValue,
}

enum FlagValue {
    Literal(u128),
    Deferred,
    Inferred,
}

#[proc_macro_attribute]
pub fn bitflags_internal(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let defaults = if attr.is_empty() { None } else {parse_defaults(attr)};
    let ast = parse_macro_input!(input as Item);
    let output = match ast {
        Item::Enum(ref item_enum) => gen_enumflags(item_enum, defaults),
        _ => Err(syn::Error::new_spanned(&ast,
                "#[bitflags] requires an enum")),
    };

    output.unwrap_or_else(|err| {
        let error = err.to_compile_error();
        quote! {
            #ast
            #error
        }
    }).into()
}

fn parse_defaults(attr: proc_macro::TokenStream) -> Option<Vec<proc_macro::Ident>> {
    let mut attr = attr.into_iter();
    // this unwrap is fine, because it must contains at least one element, because it is not empty
    let default = attr.next().unwrap();
    match default {
        TokenTree::Ident(default) => {
            if default.to_string() != "default" {
                panic!("only default parameter allowed right now");
            }
            let eq = attr.next();
            if eq.is_none() {
                panic!("default must be followed by '='");
            }
            let eq = eq.unwrap();
            if eq.to_string() != "=" {
                panic!("default must be followed by '='");
            }
            let mut defaults = vec![];
            loop {
                let default = match attr.next() {
                    None => break,
                    Some(default) => default,
                };
                match default {
                    TokenTree::Ident(default) => {
                        defaults.push(default);
                    }
                    default =>
                        panic!("default must be followed by '=' \
                        and at least one variant separated by '|'
                        '{}' is not valid identifier of an variant", default),
                }
                match attr.next() {
                    None => break,
                    Some(separator) => {
                        if separator.to_string() != "|" {
                            panic!("default must be followed by '=' \
                            and at least one variant separated by '|'
                    '{}' is not a valid separator", separator);
                        }
                    }
                }
            }
            if let Some(not_a_separator) = attr.next() {
                panic!("default must be followed by '=' \
                            and at least one variant separated by '|'
                    '{}' is not a valid separator", not_a_separator);
            }
            if defaults.is_empty() {
                panic!("default must be followed by '=' \
                and at least one variant separated by '|'");
            }
            Some(defaults)
        },
        _ => {
            panic!("only default parameter allowed right now");
        }
    }
}

/// Try to evaluate the expression given.
fn fold_expr(expr: &syn::Expr) -> Option<u128> {
    use syn::Expr;
    match expr {
        Expr::Lit(ref expr_lit) => {
            match expr_lit.lit {
                syn::Lit::Int(ref lit_int) => lit_int.base10_parse().ok(),
                _ => None,
            }
        },
        Expr::Binary(ref expr_binary) => {
            let l = fold_expr(&expr_binary.left)?;
            let r = fold_expr(&expr_binary.right)?;
            match &expr_binary.op {
                syn::BinOp::Shl(_) => {
                    u32::try_from(r).ok()
                        .and_then(|r| l.checked_shl(r))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn collect_flags<'a>(variants: impl Iterator<Item=&'a syn::Variant>)
    -> Result<Vec<Flag>, syn::Error>
{
    variants
        .map(|variant| {
            // MSRV: Would this be cleaner with `matches!`?
            match variant.fields {
                syn::Fields::Unit => (),
                _ => return Err(syn::Error::new_spanned(&variant.fields,
                    "Bitflag variants cannot contain additional data")),
            }

            let value = if let Some(ref expr) = variant.discriminant {
                if let Some(n) = fold_expr(&expr.1) {
                    FlagValue::Literal(n)
                } else {
                    FlagValue::Deferred
                }
            } else {
                FlagValue::Inferred
            };

            Ok(Flag {
                name: variant.ident.clone(),
                span: variant.span(),
                value,
            })
        })
        .collect()
}

/// Given a list of attributes, find the `repr`, if any, and return the integer
/// type specified.
fn extract_repr(attrs: &[syn::Attribute])
    -> Result<Option<Ident>, syn::Error>
{
    use syn::{Meta, NestedMeta};
    attrs.iter()
        .find_map(|attr| {
            match attr.parse_meta() {
                Err(why) => {
                    Some(Err(syn::Error::new_spanned(attr,
                        format!("Couldn't parse attribute: {}", why))))
                }
                Ok(Meta::List(ref meta)) if meta.path.is_ident("repr") => {
                    meta.nested.iter()
                        .find_map(|mi| match mi {
                            NestedMeta::Meta(Meta::Path(path)) => {
                                path.get_ident().cloned()
                                    .map(Ok)
                            }
                            _ => None
                        })
                }
                Ok(_) => None
            }
        })
        .transpose()
}

/// Check the repr and return the number of bits available
fn type_bits(ty: &Ident) -> Result<u8, syn::Error> {
    // This would be so much easier if we could just match on an Ident...
    if ty == "usize" {
        Err(syn::Error::new_spanned(ty,
            "#[repr(usize)] is not supported. Use u32 or u64 instead."))
    }
    else if ty == "i8" || ty == "i16" || ty == "i32"
            || ty == "i64" || ty == "i128" || ty == "isize" {
        Err(syn::Error::new_spanned(ty,
            "Signed types in a repr are not supported."))
    }
    else if ty == "u8" { Ok(8) }
    else if ty == "u16" { Ok(16) }
    else if ty == "u32" { Ok(32) }
    else if ty == "u64" { Ok(64) }
    else if ty == "u128" { Ok(128) }
    else {
        Err(syn::Error::new_spanned(ty,
            "repr must be an integer type for #[bitflags]."))
    }
}

/// Returns deferred checks
fn check_flag(
    type_name: &Ident,
    flag: &Flag,
) -> Result<Option<TokenStream>, syn::Error> {
    use FlagValue::*;
    match flag.value {
        Literal(n) => {
            if !n.is_power_of_two() {
                Err(syn::Error::new(flag.span,
                    "Flags must have exactly one set bit"))
            } else {
                Ok(None)
            }
        }
        Inferred => {
            Err(syn::Error::new(flag.span,
                "Please add an explicit discriminant"))
        }
        Deferred => {
            let variant_name = &flag.name;
            // MSRV: Use an unnamed constant (`const _: ...`).
            let assertion_name = syn::Ident::new(
                &format!("__enumflags_assertion_{}_{}",
                        type_name, flag.name),
                Span::call_site()); // call_site because def_site is unstable

            Ok(Some(quote_spanned!(flag.span =>
                #[doc(hidden)]
                const #assertion_name:
                    <<[(); (
                        (#type_name::#variant_name as u128).wrapping_sub(1) &
                        (#type_name::#variant_name as u128) == 0 &&
                        (#type_name::#variant_name as u128) != 0
                    ) as usize] as enumflags2::_internal::AssertionHelper>
                        ::Status as enumflags2::_internal::ExactlyOneBitSet>::X
                    = ();
            )))
        }
    }
}

fn gen_enumflags(ast: &ItemEnum, defaults: Option<Vec<proc_macro::Ident>>)
    -> Result<TokenStream, syn::Error>
{
    let ident = &ast.ident;

    let span = Span::call_site();
    // for quote! interpolation
    let variant_names =
        ast.variants.iter()
            .map(|v| &v.ident)
            .collect::<Vec<_>>();
    let repeated_name = vec![&ident; ast.variants.len()];

    let variants = collect_flags(ast.variants.iter())?;
    let deferred = variants.iter()
        .flat_map(|variant| check_flag(ident, variant).transpose())
        .collect::<Result<Vec<_>, _>>()?;

    let ty = extract_repr(&ast.attrs)?
        .ok_or_else(|| syn::Error::new_spanned(&ident,
                        "repr attribute missing. Add #[repr(u64)] or a similar attribute to specify the size of the bitfield."))?;
    let bits = type_bits(&ty)?;

    if (bits as usize) < variants.len() {
        return Err(syn::Error::new_spanned(&ty,
                   format!("Not enough bits for {} flags", variants.len())));
    }

    let std_path = quote_spanned!(span => ::enumflags2::_internal::core);

    let default = match defaults {
        None => 0,
        Some(defaults) => {
            let mut default = 0u128;
            for d in defaults {
                match ast.variants
                    .iter()
                    .find(|v| v.ident.to_string() == d.to_string()) {
                    None => panic!("{:?} is not valid varian of {:?}", d, ast.ident),
                    Some(v) => {
                        if let Some(ref expr) = v.discriminant {
                            if let Some(n) = fold_expr(&expr.1) {
                                default |= n
                            } else {
                                unimplemented!("Deferred flag value not yet supported as default");
                            }
                        } else {
                            unimplemented!("Inferred flag value not yet supported as default");
                        }
                    }
                }
            }
            default
        }
    };
    Ok(quote_spanned! {
        span =>
            #ast
            #(#deferred)*
            impl #std_path::ops::Not for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                #[inline(always)]
                fn not(self) -> Self::Output {
                    use ::enumflags2::{BitFlags, _internal::RawBitFlags};
                    unsafe { BitFlags::from_bits_unchecked(self.bits()).not() }
                }
            }

            impl #std_path::ops::BitOr for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                #[inline(always)]
                fn bitor(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlags, _internal::RawBitFlags};
                    unsafe { BitFlags::from_bits_unchecked(self.bits() | other.bits())}
                }
            }

            impl #std_path::ops::BitAnd for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                #[inline(always)]
                fn bitand(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlags, _internal::RawBitFlags};
                    unsafe { BitFlags::from_bits_unchecked(self.bits() & other.bits())}
                }
            }

            impl #std_path::ops::BitXor for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                #[inline(always)]
                fn bitxor(self, other: Self) -> Self::Output {
                    #std_path::convert::Into::<Self::Output>::into(self) ^ #std_path::convert::Into::<Self::Output>::into(other)
                }
            }

            impl ::enumflags2::_internal::RawBitFlags for #ident {
                type Numeric = #ty;

                const EMPTY: Self::Numeric = 0;

                const DEFAULT: Self::Numeric = #default as #ty;

                const ALL_BITS: Self::Numeric =
                    0 #(| (#repeated_name::#variant_names as #ty))*;

                const FLAG_LIST: &'static [Self] =
                    &[#(#repeated_name::#variant_names),*];

                const BITFLAGS_TYPE_NAME : &'static str =
                    concat!("BitFlags<", stringify!(#ident), ">");

                fn bits(self) -> Self::Numeric {
                    self as #ty
                }
            }

            impl ::enumflags2::BitFlag for #ident {}
    })
}
