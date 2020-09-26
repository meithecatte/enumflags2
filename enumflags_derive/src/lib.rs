#![recursion_limit = "2048"]
extern crate proc_macro;
#[macro_use]
extern crate quote;

use std::convert::TryFrom;
use syn::{Data, Ident, DeriveInput, DataEnum, spanned::Spanned};
use proc_macro2::{TokenStream, Span};

struct Flag {
    name: Ident,
    span: Span,
    value: FlagValue,
}

enum FlagValue {
    Literal(u64),
    Deferred,
    Inferred,
}

#[proc_macro_attribute]
pub fn bitflags_internal(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let impls = match ast.data {
        Data::Enum(ref data) => {
            gen_enumflags(&ast.ident, &ast, data)
        }
        Data::Struct(ref data) => {
            Err(syn::Error::new_spanned(data.struct_token, "#[bitflags] requires an enum"))
        }
        Data::Union(ref data) => {
            Err(syn::Error::new_spanned(data.union_token, "#[bitflags] requires an enum"))
        }
    };

    let impls = impls.unwrap_or_else(|err| err.to_compile_error());
    let combined = quote! {
        #ast
        #impls
    };
    combined.into()
}

/// Try to evaluate the expression given.
fn fold_expr(expr: &syn::Expr) -> Option<u64> {
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
    -> Result<Option<syn::Ident>, syn::Error>
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
                        (#type_name::#variant_name as u64).wrapping_sub(1) &
                        (#type_name::#variant_name as u64) == 0 &&
                        (#type_name::#variant_name as u64) != 0
                    ) as usize] as enumflags2::_internal::AssertionHelper>
                        ::Status as enumflags2::_internal::ExactlyOneBitSet>::X
                    = ();
            )))
        }
    }
}

fn gen_enumflags(ident: &Ident, item: &DeriveInput, data: &DataEnum)
    -> Result<TokenStream, syn::Error>
{
    let span = Span::call_site();
    // for quote! interpolation
    let variant_names = data.variants.iter().map(|v| &v.ident);
    let variant_count = data.variants.len();

    let repeated_name = std::iter::repeat(&ident);

    let variants = collect_flags(data.variants.iter())?;
    let deferred = variants.iter()
        .flat_map(|variant| check_flag(ident, variant).transpose())
        .collect::<Result<Vec<_>, _>>()?;

    let ty = extract_repr(&item.attrs)?
        .ok_or_else(|| syn::Error::new_spanned(&ident,
                        "repr attribute missing. Add #[repr(u64)] or a similar attribute to specify the size of the bitfield."))?;
    let std_path = quote_spanned!(span => ::enumflags2::_internal::core);
    let all = if variant_count == 0 {
        quote!(0)
    } else {
        let repeated_name = repeated_name.clone();
        let variant_names = variant_names.clone();
        quote!(#(#repeated_name::#variant_names as #ty)|*)
    };

    Ok(quote_spanned! {
        span => #(#deferred)*
            impl #std_path::ops::Not for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                fn not(self) -> Self::Output {
                    use ::enumflags2::{BitFlags, _internal::RawBitFlags};
                    unsafe { BitFlags::new(self.bits()).not() }
                }
            }

            impl #std_path::ops::BitOr for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                fn bitor(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlags, _internal::RawBitFlags};
                    unsafe { BitFlags::new(self.bits() | other.bits())}
                }
            }

            impl #std_path::ops::BitAnd for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                fn bitand(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlags, _internal::RawBitFlags};
                    unsafe { BitFlags::new(self.bits() & other.bits())}
                }
            }

            impl #std_path::ops::BitXor for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                fn bitxor(self, other: Self) -> Self::Output {
                    #std_path::convert::Into::<Self::Output>::into(self) ^ #std_path::convert::Into::<Self::Output>::into(other)
                }
            }

            impl ::enumflags2::_internal::RawBitFlags for #ident {
                type Type = #ty;

                fn all_bits() -> Self::Type {
                    // make sure it's evaluated at compile time
                    const VALUE: #ty = #all;
                    VALUE
                }

                fn bits(self) -> Self::Type {
                    self as #ty
                }

                fn flag_list() -> &'static [Self] {
                    const VARIANTS: [#ident; #variant_count] = [#(#repeated_name :: #variant_names),*];
                    &VARIANTS
                }

                fn bitflags_type_name() -> &'static str {
                    concat!("BitFlags<", stringify!(#ident), ">")
                }
            }

            impl ::enumflags2::BitFlag for #ident {}
    })
}
