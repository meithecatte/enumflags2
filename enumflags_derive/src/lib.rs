#![recursion_limit = "2048"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro2;
use syn::{Data, Ident, DeriveInput, DataEnum};
use proc_macro2::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use std::convert::From;

#[proc_macro_derive(EnumFlags)]
pub fn derive_enum_flags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(ref data) => gen_enumflags(&ast.ident, &ast, data).into(),
        _ => panic!("`derive(EnumFlags)` may only be applied to enums"),
    }
}

fn max_value_of(ty: &str) -> Option<usize> {
    match ty {
        "u8" => Some(u8::max_value() as usize),
        "u16" => Some(u16::max_value() as usize),
        "u32" => Some(u32::max_value() as usize),
        "u64" => Some(u64::max_value() as usize),
        "usize" => Some(usize::max_value()),
        _ => None,
    }
}

fn fold_expr(expr: &syn::Expr) -> u64 {
    use syn::Expr;
    match expr{
        &Expr::Lit(ref expr_lit) => {
            match expr_lit.lit {
                syn::Lit::Int(ref lit_int) => lit_int.base10_parse().expect("Int literal out of range"),
                _ => panic!("Only Int literals are supported")
            }
        },
        &Expr::Binary(ref expr_binary) => {
            let l = fold_expr(&expr_binary.left);
            let r = fold_expr(&expr_binary.right);
            match &expr_binary.op {
                syn::BinOp::Shl(_) => l << r,
                op => panic!("{} not supported", op.to_token_stream())
            }

        }
        _ => panic!("Only literals are supported")
    }
}

fn extract_repr(attrs: &[syn::Attribute]) -> Option<syn::Ident> {
    attrs
        .iter()
        .filter_map(|a| {
            if let syn::Meta::List(ref meta) = a.parse_meta().expect("Metalist") {
                if meta.path.is_ident("repr") {
                    return meta.nested
                        .iter()
                        .filter_map(|mi| {
                            if let syn::NestedMeta::Meta(syn::Meta::Path(path)) =
                                mi
                            {
                                return path.get_ident().cloned();
                            }
                            None
                        })
                        .nth(0);
                }
            }
            None
        })
        .nth(0)
}
fn gen_enumflags(ident: &Ident, item: &DeriveInput, data: &DataEnum) -> TokenStream {
    let span  = Span::call_site();
    let variants = data.variants.iter().map(|v| &v.ident);
    let flag_values: Vec<_> = data.variants.iter()
        .map(|v| v.discriminant.as_ref().map(|d| fold_expr(&d.1)).expect("No discriminant")).collect();
    let variants_len = flag_values.len();
    assert!(flag_values.iter().find(|&&v| v == 0).is_none(), "Null flag is not allowed");
    let names = flag_values.iter().map(|_| &ident);
    let ty = extract_repr(&item.attrs).unwrap_or(Ident::new("usize", span));
    let max_flag_value = flag_values.iter().max().unwrap();
    let max_allowed_value = max_value_of(&ty.to_string()).expect(&format!("{} is not supported", ty));
    assert!(
        *max_flag_value as usize <= max_allowed_value,
        format!(
            "Value '0b{val:b}' is too big for an {ty}",
            val = max_flag_value,
            ty = ty
        )
    );
    let wrong_flag_values: &Vec<_> = &flag_values
        .iter()
        .zip(variants.clone())
        .filter(|&(&val, _)| flag_values.iter().filter(|&&v| v & val != 0).count() > 1)
        .map(|(value, variant)| {
            format!(
                "{name}::{variant} = 0b{value:b}",
                name = ident,
                variant = variant,
                value = value
            )
        })
        .collect();
    assert!(
        wrong_flag_values.len() == 0,
        format!(
            "The following flags are not unique: {data:?}",
            data = wrong_flag_values
        )
    );
    let std_path = quote_spanned!(span=> ::enumflags2::_internal::core);
    let scope_ident = Ident::new(&format!("__scope_enumderive_{}",
                                          item.ident.to_string().to_lowercase()), span);
    quote_spanned!{
        span =>
        mod #scope_ident {
            use super::#ident;

            const VARIANTS: [#ident; #variants_len] = [#(#names :: #variants, )*];

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

                fn all() -> Self::Type {
                    (#(#flag_values)|*) as #ty
                }

                fn bits(self) -> Self::Type {
                    self as #ty
                }

                fn flag_list() -> &'static [Self] {
                    &VARIANTS
                }

                fn bitflags_type_name() -> &'static str {
                    concat!("BitFlags<", stringify!(#ident), ">")
                }
            }
        }
    }
}
