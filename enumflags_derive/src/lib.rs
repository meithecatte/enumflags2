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

#[proc_macro_derive(EnumFlags_internal)]
pub fn derive_enum_flags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(ref data) => gen_enumflags(&ast.ident, &ast, data).into(),
        _ => panic!("`derive(EnumFlags)` may only be applied to enums"),
    }
}

fn max_value_of(ty: &str) -> Option<u64> {
    match ty {
        "u8" => Some(u8::max_value() as u64),
        "u16" => Some(u16::max_value() as u64),
        "u32" => Some(u32::max_value() as u64),
        "u64" => Some(u64::max_value() as u64),
        "usize" => Some(usize::max_value() as u64),
        _ => None,
    }
}

fn fold_expr(expr: &syn::Expr) -> u64 {
    use syn::Expr;
    match expr {
        Expr::Lit(ref expr_lit) => {
            match expr_lit.lit {
                syn::Lit::Int(ref lit_int) => lit_int.base10_parse().expect("Int literal out of range"),
                _ => panic!("Only Int literals are supported")
            }
        },
        Expr::Binary(ref expr_binary) => {
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
    let flag_values: Vec<_> =
        data.variants.iter()
        .map(|v| v.discriminant.as_ref()
                 .map(|d| fold_expr(&d.1)).expect("No discriminant"))
        .collect();
    let variants_len = flag_values.len();
    let names = flag_values.iter().map(|_| &ident);
    let ty = extract_repr(&item.attrs).unwrap_or(Ident::new("usize", span));
    let max_allowed_value = max_value_of(&ty.to_string()).expect(&format!("{} is not supported", ty));

    let mut flags_seen = 0;
    for (&flag, variant) in flag_values.iter().zip(variants.clone()) {
        if flag > max_allowed_value {
            panic!("Value {:#b} is too big for an {}",
                   flag, ty
            );
        } else if flag == 0 || !flag.is_power_of_two() {
            panic!("Each flag must have exactly one bit set, and {ident}::{variant} = {flag:#b} doesn't",
                   ident = ident,
                   variant = variant,
                   flag = flag
            );
        } else if flags_seen & flag != 0 {
            panic!("Flag {} collides with {}",
                   variant,
                   flag_values.iter()
                       .zip(variants.clone())
                       .find(|(&other_flag, _)| flag == other_flag)
                       .unwrap()
                       .1
            );
        }

        flags_seen |= flag;
    }

    let std_path = quote_spanned!(span=> ::enumflags2::_internal::core);
    quote_spanned!{
        span =>
            impl #std_path::ops::Not for #ident {
                type Output = <Self as ::enumflags2::BitFlag>::Flags;
                fn not(self) -> Self::Output {
                    use ::enumflags2::{BitFlagExt, EnumFlags};
                    self.into_flags().not()
                }
            }

            impl #std_path::ops::BitOr for #ident {
                type Output = <Self as ::enumflags2::BitFlag>::Flags;
                fn bitor(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlagExt, EnumFlags};
                    self.into_flags() | other
                }
            }

            impl #std_path::ops::BitAnd for #ident {
                type Output = <Self as ::enumflags2::BitFlag>::Flags;
                fn bitand(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlagExt, EnumFlags};
                    self.into_flags() & other
                }
            }

            impl #std_path::ops::BitXor for #ident {
                type Output = <Self as ::enumflags2::BitFlag>::Flags;
                fn bitxor(self, other: Self) -> Self::Output {
                    use ::enumflags2::{BitFlagExt, EnumFlags};
                    self.into_flags() ^ other
                }
            }

            unsafe impl ::enumflags2::repr::BitFlagRepr<#ident> for #ident {
                #[inline]
                fn into_repr(self) -> #ty {
                    self as #ty
                }

                #[inline]
                fn get_repr(&self) -> #ty {
                    *self as #ty
                }

                fn from_repr(bits: #ty) -> #std_path::result::Result<Self, #ty> {
                    use ::enumflags2::BitFlag;
                    use #std_path::result::Result;
                    if bits & !Self::ALL_BITS == 0 && bits.count_ones() == 1 {
                        // NOTE: avoid a match by relying on the invariant that
                        //       a flag may not contain more than one bit
                        Result::Ok(unsafe { Self::from_repr_unchecked(bits) })
                    } else {
                        Result::Err(bits)
                    }
                }

                unsafe fn from_repr_unchecked(bits: #ty) -> Self {
                    #std_path::mem::transmute(bits)
                }
            }

            impl ::enumflags2::BitFlag for #ident {
                type Type = #ty;
                type Flags = ::enumflags2::BitFlags<Self>;

                const ALL_BITS: Self::Type = (#(#flag_values)|*) as #ty;

                #[inline]
                fn flag_list() -> &'static [Self] {
                    const VARIANTS: [#ident; #variants_len] = [#(#names :: #variants, )*];
                    &VARIANTS
                }

                #[inline]
                fn bitflags_type_name() -> &'static str {
                    concat!("BitFlags<", stringify!(#ident), ">")
                }
            }
    }
}
