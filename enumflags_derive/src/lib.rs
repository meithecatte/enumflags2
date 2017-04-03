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

    #[cfg(feature = "std")]
    let gen_std = true;
    #[cfg(not(feature = "std"))]
    let gen_std = false;
    let quote_tokens = match ast.body {
        Body::Enum(ref data) => gen_enumflags(&ast.ident, &ast, data, gen_std),
        _ => panic!("`derive(EnumFlags)` may only be applied to enums"),
    };

    //println!("{:?}", quote_tokens);
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

fn gen_enumflags(ident: &Ident, item: &MacroInput, data: &Vec<Variant>, gen_std: bool) -> Tokens {
    let variants: Vec<_> = data.iter().map(|v| v.ident.clone()).collect();
    let variants_ref = &variants;
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
    let flag_value_names: &Vec<_> =
        &flag_values.iter().map(|val| Ident::new(format!("{}", val))).collect();
    let names: Vec<_> = flag_values.iter().map(|_| ident.clone()).collect();
    let names_ref = &names;
    assert!(variants.len() == flag_values.len(),
            "At least one variant was not initialized explicity with a value.");
    let ty_attr = item.attrs
        .iter()
        .filter_map(|a| {
            if let syn::MetaItem::List(ref ident, ref items) = a.value {
                if ident.as_ref() == "repr" {
                    return items.iter().filter_map(|mi| {
                        if let &syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref ident)) = mi {
                                return Some(Ident::new(ident.clone()));
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
    let wrong_flag_values: &Vec<_> = &flag_values.iter()
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
        .filter(|&(_, count)| count > 0)
        .map(|(index, _)| {
            format!("{name}::{variant} = 0b{value:b}",
                    name = ident,
                    variant = variants_ref[index],
                    value = flag_values[index])
        })
        .collect();
    assert!(wrong_flag_values.len() == 0,
            format!("The following flags are not unique: {data:?}",
                     data = wrong_flag_values));
    let inner_name = Ident::new(format!("Inner{}", ident));
    let std = if gen_std {
        quote! {
            impl #ident{
                pub fn from_bitflag(bitflag: ::enumflags::BitFlags<#ident>) -> Vec<#ident> {
                    #flag_values_ref1.iter().filter_map(|val|{
                        let val = *val as #ty & bitflag.bits();
                        match val {
                            #(#flag_value_names => Some(#names_ref :: #variants_ref),)*
                            _ => None
                        }
                    }).collect()
                }
            }
        }
    }
    else{
        quote!{}
    };
    quote!{
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct #inner_name(#ty);

        impl ::enumflags::__core::ops::BitOr for #inner_name{
            type Output = Self;
            fn bitor(self, other: Self) -> Self{
                #inner_name(self.0 | other.0)
            }
        }

        impl ::enumflags::__core::ops::BitAnd for #inner_name{
            type Output = Self;
            fn bitand(self, other: Self) -> Self{
                #inner_name(self.0 & other.0)
            }
        }

        impl ::enumflags::__core::ops::BitXor for #inner_name{
            type Output = Self;
            fn bitxor(self, other: Self) -> Self{
                #inner_name(self.0 ^ other.0)
            }
        }

        impl ::enumflags::__core::ops::Not for #inner_name{
            type Output = Self;
            fn not(self) -> Self{
                #inner_name(!self.0)
            }
        }

        impl ::enumflags::InnerBitFlags for #inner_name{
            type Type = #ty;
            fn all() -> Self {
               let val = (#(#flag_values_ref1)|*) as #ty;
               #inner_name(val)
            }

            fn empty() -> Self {
                #inner_name(0)
            }

            fn is_empty(self) -> bool {
                self == Self::empty()
            }

            fn is_all(self) -> bool {
                self == Self::all()
            }

            fn bits(self) -> Self::Type {
                self.0
            }

            fn intersects(self, other: Self) -> bool{
                (self & other).0 > 0
            }

            fn contains(self, other: Self) -> bool{
                (self & other) == other
            }

            fn not(self) -> Self {
                #inner_name(!self.0)
            }

            fn from_bits(bits: #ty) -> Option<Self> {
                if #inner_name(bits) & Self::all().not() == Self::empty(){
                    Some(#inner_name(bits))
                }
                else{
                    None
                }
            }

            fn from_bits_truncate(bits: #ty) -> Self {
                #inner_name(bits) & Self::all()
            }

            fn insert(&mut self, other: Self){
                let new_val = *self | other;
                *self = new_val;
            }

            fn remove(&mut self, other: Self){
                let new_val = *self | other.not();
                *self = new_val;
            }

            fn toggle(&mut self, other: Self){
                let new_val = *self ^ other;
                *self = new_val;
            }
        }

        impl Into<#ty> for #inner_name{
            fn into(self) -> #ty{
                self.0 as #ty
            }
        }

        impl Into<#inner_name> for #ident{
            fn into(self) -> #inner_name{
                #inner_name(self.into())
            }
        }

        impl Into<#ty> for #ident{
            fn into(self) -> #ty{
                self as #ty
            }
        }

        impl Into<::enumflags::BitFlags<#ident>> for #inner_name{
            fn into(self) -> ::enumflags::BitFlags<#ident> {
                unsafe{ ::enumflags::BitFlags::new(self)}
            }
        }

        impl ::enumflags::__core::fmt::Debug for #inner_name{
            fn fmt(&self, fmt: &mut ::enumflags::__core::fmt::Formatter) -> ::enumflags::__core::fmt::Result {
                let v = #flag_values_ref1;
                let v = v.iter().filter_map(|val|{
                    let val: #ty = *val as #ty & self.0;
                    match val {
                        #(#flag_value_names => Some(#names_ref :: #variants_ref),)*
                        _ => None
                    }
                });
                write!(fmt, "0b{:b}, Flags::", self.0);
                fmt.debug_list().entries(v).finish()
            }
        }

        #std

        impl #ident{
           pub fn max_bitflag() -> ::enumflags::BitFlags<#ident> {
               let val = (#(#flag_values_ref1)|*) as #ty;
               unsafe{ ::enumflags::BitFlags::new(#inner_name(val)) }
           }

           pub fn empty_bitflag() -> ::enumflags::BitFlags<#ident>{
               unsafe{ ::enumflags::BitFlags::new(#inner_name(0)) }
           }
        }

        impl From<#ident> for ::enumflags::BitFlags<#ident> {
            fn from(t: #ident) -> Self {
                unsafe { ::enumflags::BitFlags::new(t.into()) }
            }
        }

        impl ::enumflags::__core::ops::BitOr for #ident {
            type Output = ::enumflags::BitFlags<#ident>;
            fn bitor(self, other: Self) -> Self::Output {
                let l: #inner_name = self.into();
                let r: #inner_name = other.into();
                (l | r).into()
            }
        }

        impl ::enumflags::__core::ops::BitAnd for #ident {
            type Output = ::enumflags::BitFlags<#ident>;
            fn bitand(self, other: Self) -> Self::Output {
                let l: #inner_name = self.into();
                let r: #inner_name = other.into();
                (l & r).into()
            }
        }

        impl ::enumflags::__core::ops::BitXor for #ident {
            type Output = ::enumflags::BitFlags<#ident>;
            fn bitxor(self, other: Self) -> Self::Output {
                let l: #inner_name = self.into();
                let r: #inner_name = other.into();
                (l ^ r).into()
            }
        }

        impl ::enumflags::__core::ops::Not for #ident {
            type Output = ::enumflags::BitFlags<#ident>;
            fn not(self) -> Self::Output {
                let r: #inner_name = self.into();
                (!r).into()
            }
        }

        impl ::enumflags::EnumFlagSize for #ident {
            type Size = #inner_name;
        }
    }
}
