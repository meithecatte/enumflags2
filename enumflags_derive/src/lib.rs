#![recursion_limit = "2048"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
use syn::{Body, Ident, MacroInput, Variant};
use quote::Tokens;
use proc_macro::TokenStream;

#[proc_macro_derive(EnumFlags, attributes(EnumFlags))]
pub fn derive_enum_flags(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let ast = syn::parse_macro_input(&input).unwrap();

    #[cfg(not(feature = "nostd"))]
    let gen_std = true;
    #[cfg(feature = "nostd")]
    let gen_std = false;
    let quote_tokens = match ast.body {
        Body::Enum(ref data) => gen_enumflags(&ast.ident, &ast, data, gen_std),
        _ => panic!("`derive(EnumFlags)` may only be applied to enums"),
    };
    quote_tokens.parse().unwrap()
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

fn gen_enumflags(ident: &Ident, item: &DeriveInput, data: &DataEnum, gen_std: bool) -> Tokens {
    //println!("{:#?}", item);
    let span  = Span::call_site();
    let variants: Vec<_> = data.variants.iter().map(|v| v.ident.clone()).collect();
    let variants_ref = &variants;
    let flag_values: Vec<_> = data.variants.iter()
        .map(|v| v.discriminant.as_ref().map(|d| fold_expr(&d.1)).expect("No discriminant")).collect();
    let flag_values_ref1 = &flag_values;
    let flag_value_names: &Vec<_> = &flag_values
        .iter()
        .map(|val| syn::Index::from(*val as usize))
        .collect();
    let names: Vec<_> = flag_values.iter().map(|_| ident.clone()).collect();
    let ty = Ident::new("u8", span);
    let names_ref = &names;
    assert!(
        variants.len() == flag_values.len(),
        "At least one variant was not initialized explicity with a value."
    );
    //println!("{:#?}", item.attrs[0].interpret_meta());
    
    // let ty_attr = item.attrs
    //     .iter()
    //     .filter_map(|a| {
    //         if let syn::Meta::List(ref ident, ref items) = a.interpret_meta().expect("Metalist") {
    //             if ident.as_ref() == "repr" {
    //                 return items
    //                     .iter()
    //                     .filter_map(|mi| {
    //                         if let &syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref ident)) =
    //                             mi
    //                         {
    //                             return Some(Ident::new(ident.clone()));
    //                         }
    //                         None
    //                     })
    //                     .nth(0);
    //             }
    //         }
    //         None
    //     })
    //     .nth(0);
    // let ty = ty_attr.unwrap_or(Ident::new("usize"));
    // let max_flag_value = flag_values.iter().max().unwrap();
    // let max_allowed_value = max_value_of(ty.as_ref()).expect(&format!("{} is not supported", ty));
    // assert!(
    //     *max_flag_value as usize <= max_allowed_value,
    //     format!(
    //         "Value '0b{val:b}' is too big for an {ty}",
    //         val = max_flag_value,
    //         ty = ty
    //     )
    // );
    // let wrong_flag_values: &Vec<_> = &flag_values
    //     .iter()
    //     .enumerate()
    //     .map(|(i, &val)| {
    //         (
    //             i,
    //             flag_values
    //                 .iter()
    //                 .enumerate()
    //                 .fold(0u32, |acc, (other_i, &other_val)| {
    //                     if other_i == i || other_val > 0 && other_val & val == 0 {
    //                         acc
    //                     } else {
    //                         acc + 1
    //                     }
    //                 }),
    //         )
    //     })
    //     .filter(|&(_, count)| count > 0)
    //     .map(|(index, _)| {
    //         format!(
    //             "{name}::{variant} = 0b{value:b}",
    //             name = ident,
    //             variant = variants_ref[index],
    //             value = flag_values[index]
    //         )
    //     })
    //     .collect();
    // assert!(
    //     wrong_flag_values.len() == 0,
    //     format!(
    //         "The following flags are not unique: {data:?}",
    //         data = wrong_flag_values
    //     )
    // );
    #[cfg(not(feature = "nostd"))]
    let std_path = Ident::new("std", span);
    #[cfg(feature = "nostd")]
    let std_path = Ident::from("::core");
    // let std: quote::Tokens = if gen_std {
    //     quote_spanned! {
    //         span =>
    //         impl #ident{
    //             pub fn from_bitflag(bitflag: ::enumflags::BitFlags<#ident>) -> Vec<#ident> {
    //                 #flag_values_ref1.iter().filter_map(|val|{
    //                     let val = *val as #ty & bitflag.bits();
    //                     match val {
    //                         #(#flag_value_names => Some(#names_ref :: #variants_ref),)*
    //                         _ => None
    //                     }
    //                 }).collect()
    //             }
    //         }
    //     }
    // } else {
    //     quote!{}
    // };
    let std = quote!{};
    quote!{
        impl #std_path::ops::Not for #ident{
            type Output = ::enumflags::BitFlags<#ident>;
            fn not(self) -> Self::Output {
                use ::enumflags::{BitFlags, RawBitFlags};
                unsafe { BitFlags::new(self.bits()).not() }
            }
        }

        impl #std_path::ops::BitOr for #ident{
            type Output = ::enumflags::BitFlags<#ident>;
            fn bitor(self, other: Self) -> Self::Output {
                use ::enumflags::{BitFlags, RawBitFlags};
                unsafe { BitFlags::new(self.bits() | other.bits())}
            }
        }

        impl #std_path::ops::BitAnd for #ident{
            type Output = ::enumflags::BitFlags<#ident>;
            fn bitand(self, other: Self) -> Self::Output {
                use ::enumflags::{BitFlags, RawBitFlags};
                unsafe { BitFlags::new(self.bits() & other.bits())}
            }
        }

        impl #std_path::ops::BitXor for #ident{
            type Output = ::enumflags::BitFlags<#ident>;
            fn bitxor(self, other: Self) -> Self::Output {
                Into::<Self::Output>::into(self) ^ Into::<Self::Output>::into(other)
            }
        }

        impl ::enumflags::BitFlagsFmt for #ident {
            fn fmt(flags: ::enumflags::BitFlags<#ident>,
                   fmt: &mut #std_path::fmt::Formatter)
                   -> #std_path::fmt::Result {
                use ::enumflags::RawBitFlags;
                let v:Vec<&str> =
                    [#((#names_ref :: #variants_ref).bits(),)*]
                    .iter()
                    .filter_map(|val|{
                        let val: #ty = *val as #ty & flags.bits();
                        match val {
                            #(#flag_value_names => Some(stringify!(#variants_ref)),)*
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

        impl ::enumflags::RawBitFlags for #ident {
            type Type = #ty;

            fn all() -> Self::Type {
               (#(#flag_values_ref1)|*) as #ty
            }

            fn bits(self) -> Self::Type {
                self as #ty
            }
        }


        #std
    }
}
