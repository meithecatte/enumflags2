#![recursion_limit="2048"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use syn::{Body, Ident, MacroInput, Variant};
use quote::Tokens;
use proc_macro::TokenStream;

#[proc_macro_derive(EnumFlags, attributes(EnumFlags))]
pub fn derive_enum_flags(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let ast = syn::parse_macro_input(&input).unwrap();

    let quote_tokens = match ast.body {
        Body::Enum(ref data) => gen_enumflags(&ast.ident, &ast, data),
        _ => panic!("`derive(Enum*)` may only be applied to enum items"),
    };

    println!("{:?}", quote_tokens);
    quote_tokens.parse().unwrap()
}

fn max_value_of(ty: &str) -> Option<usize> {
    match ty {
        "u8" => Some(u8::max_value() as usize),
        "u16" => Some(u16::max_value() as usize),
        "u32" => Some(u32::max_value() as usize),
        "u64" => Some(u64::max_value() as usize),
        _ => None,
    }
}

fn gen_enumflags(ident: &Ident, item: &MacroInput, data: &Vec<Variant>) -> Tokens {
    let variants: Vec<_> = data.iter().map(|v| v.ident.clone()).collect();
    let flag_values: Vec<_> = data.iter()
        .filter_map(|v| {
            if let Some(syn::ConstExpr::Lit(syn::Lit::Int(flag, _))) = v.discriminant {
                Some(flag)
            } else {
                None
            }
        })
        .collect();
    let flag_values_ref1 = &flag_values;
    let flag_value_names: Vec<_> =
        flag_values.iter().map(|val| Ident::new(format!("{}", val))).collect();
    let names: Vec<_> = flag_values.iter().map(|_| ident.clone()).collect();
    assert!(variants.len() == flag_values.len(),
            "At least one variant was not initialized explicity with a value.");
    let ty_attr = item.attrs
        .iter()
        .filter_map(|a| {
            if let syn::MetaItem::List(ref ident, ref items) = a.value {
                if ident.as_ref() == "EnumFlags" {
                    return items.iter().filter_map(|mi| {
                        if let &syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref ident,
                                                        syn::Lit::Str(ref ty_name, _))) = mi {
                            if ident.as_ref() == "ty" {
                                return Some(Ident::new(ty_name.clone()));
                            }
                        }
                        None
                    }).nth(0);
                }
            }
            None
        })
        .nth(0);
    let ty = ty_attr.unwrap_or(Ident::new("usize"));
    let max_flag_value = flag_values.iter().max().unwrap();
    let max_allowed_value = max_value_of(ty.as_ref()).expect(&format!("{} is not supported", ty));
    assert!(*max_flag_value as usize <= max_allowed_value,
            format!("Value '0b{val:b}' is too big for an {ty}",
                    val = max_flag_value,
                    ty = ty));
    let mut wrong_flag_values: Vec<_> = flag_values.iter()
        .enumerate()
        .map(|(i, &val)| {
            (i,
             flag_values.iter().enumerate().fold(0u32, |acc, (other_i, &other_val)| {
                if other_i == i || other_val > 0 && other_val & val == 0 {
                    acc
                } else {
                    acc + 1
                }
            }))
        })
        .collect();
    wrong_flag_values.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let index_wrong_val = wrong_flag_values.iter().nth(0);

    if let Some(&(index, count)) = index_wrong_val {
        assert!(count <= 1,
                format!("{name}::{variant} = 0b{value:b} is not unique value.",
                        name = ident,
                        variant = variants[index],
                        value = flag_values[index]));

    }
    quote!{
        impl Into<#ty> for #ident{
            fn into(self) -> #ty{
                self as #ty
            }
        }

        impl #ident{
            pub fn from_bitflag(bitflag: enumflags::Bitflag<#ident>) -> Vec<#ident> {
                #flag_values_ref1.iter().filter_map(|val|{
                    let val = *val as #ty & bitflag.bits();
                    match val {
                        #(#flag_value_names => Some(#names :: #variants),)*
                        _ => None
                    }
                }).collect()
            }

            pub fn max_bitflag() -> enumflags::Bitflag<#ident> {
                unsafe{ Bitflag::new((#(#flag_values_ref1)|*) as #ty) }
            }

            pub fn empty_bitflag() -> enumflags::Bitflag<#ident>{
                unsafe{ Bitflag::new(0) }
            }
        }

        impl From<#ident> for enumflags::Bitflag<#ident> {
            fn from(t: #ident) -> Self {
                unsafe { enumflags::Bitflag::new(t as <#ident as EnumFlagSize>::Size) }
            }
        }

        impl ::std::ops::BitOr for #ident {
            type Output = enumflags::Bitflag<#ident>;
            fn bitor(self, other: Self) -> Self::Output {
                let l: enumflags::Bitflag<#ident> = self.into();
                let r: enumflags::Bitflag<#ident> = other.into();
                l | r
            }
        }

        impl enumflags::EnumFlagSize for #ident {
            type Size = #ty;
        }
    }
}
