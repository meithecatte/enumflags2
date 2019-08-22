#![recursion_limit = "2048"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro2;
use syn::{Data, Ident, DeriveInput, DataEnum};
use proc_macro2::TokenStream;
use proc_macro2::Span;

#[proc_macro_derive(EnumFlags, attributes(EnumFlags))]
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
                syn::Lit::Int(ref lit_int) => lit_int.value(),
                _ => panic!("Only Int literals are supported")
            }
        },
        &Expr::Binary(ref expr_binary) => {
            let l = fold_expr(&expr_binary.left);
            let r = fold_expr(&expr_binary.right);
            match expr_binary.op {
                syn::BinOp::Shl(_) => l << r,
                op => panic!("{:?} not supported", op)
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
                if meta.ident.to_string() == "repr" {
                    return meta.nested
                        .iter()
                        .filter_map(|mi| {
                            if let &syn::NestedMeta::Meta(syn::Meta::Word(ref ident)) =
                                mi
                            {
                                return Some(ident.clone());
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
    let variants: &Vec<_> = &data.variants.iter().map(|v| v.ident.clone()).collect();
    let flag_values: &Vec<_> = &data.variants.iter()
        .map(|v| v.discriminant.as_ref().map(|d| fold_expr(&d.1)).expect("No discriminant")).collect();
    assert!(flag_values.iter().find(|&&v| v == 0).is_none(), "Null flag is not allowed");
    let flag_value_names: &Vec<_> = &flag_values
        .iter()
        .map(|&val| syn::LitInt::new(val, syn::IntSuffix::None, span))
        .collect();
    let names: &Vec<_> = &flag_values.iter().map(|_| ident.clone()).collect();
    assert!(
        variants.len() == flag_values.len(),
        "At least one variant was not initialized explicity with a value."
    );
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
        .enumerate()
        .map(|(i, &val)| {
            (
                i,
                flag_values
                    .iter()
                    .enumerate()
                    .fold(0u32, |acc, (other_i, &other_val)| {
                        if other_i == i || other_val > 0 && other_val & val == 0 {
                            acc
                        } else {
                            acc + 1
                        }
                    }),
            )
        })
        .filter(|&(_, count)| count > 0)
        .map(|(index, _)| {
            format!(
                "{name}::{variant} = 0b{value:b}",
                name = ident,
                variant = variants[index],
                value = flag_values[index]
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
    let scope_ident = Ident::new(&format!("__scope_enumderive_{}",
                                          item.ident.to_string().to_lowercase()), span);
    quote_spanned!{
        span =>
        mod #scope_ident {
            extern crate core;
            use super::#ident;
            impl core::ops::Not for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                fn not(self) -> Self::Output {
                    use ::enumflags2::{BitFlags, RawBitFlags};
                    unsafe { BitFlags::new(self.bits()).not() }
                }
            }

            impl core::ops::BitOr for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                fn bitor(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlags, RawBitFlags};
                    unsafe { BitFlags::new(self.bits() | other.bits())}
                }
            }

            impl core::ops::BitAnd for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                fn bitand(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlags, RawBitFlags};
                    unsafe { BitFlags::new(self.bits() & other.bits())}
                }
            }

            impl core::ops::BitXor for #ident {
                type Output = ::enumflags2::BitFlags<#ident>;
                fn bitxor(self, other: Self) -> Self::Output {
                    Into::<Self::Output>::into(self) ^ Into::<Self::Output>::into(other)
                }
            }

            impl ::enumflags2::BitFlagsFmt for #ident {
                fn fmt(flags: ::enumflags2::BitFlags<#ident>,
                       fmt: &mut core::fmt::Formatter)
                       -> core::fmt::Result {
                    use ::enumflags2::RawBitFlags;
                    let v:Vec<&str> =
                        [#((#names :: #variants).bits(),)*]
                        .iter()
                        .filter_map(|val|{
                            let val: #ty = *val as #ty & flags.bits();
                            match val {
                                #(#flag_value_names => Some(stringify!(#variants)),)*
                                _ => None
                            }
                        })
                        .collect();
                    write!(fmt, "BitFlags<{}>(0b{:b}, [{}]) ",
                           stringify!(#ident),
                           flags.bits(),
                           v.join(", "))
                }
            }

            impl ::enumflags2::RawBitFlags for #ident {
                type Type = #ty;

                fn all() -> Self::Type {
                    (#(#flag_values)|*) as #ty
                }

                fn bits(self) -> Self::Type {
                    self as #ty
                }

                fn flag_list() -> &'static [Self] {
                    &[#(#names::#variants,)*]
                }
            }
        }
    }
}
