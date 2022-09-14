// Based on https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs

#![feature(let_else)]

use proc_macro2::{Span, TokenStream};

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DataStruct, DeriveInput,
    GenericParam, Generics, Ident, Index,
};

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let Data::Struct(ref struct_data) = input.data else {
        panic!("Deserialize can only be derived on structs");
    };

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let num_bytes = num_bytes_const(struct_data);
    let alignment = alignment_const(struct_data);
    let create_struct = create_struct_expr(struct_data);

    let expanded = quote! {
        impl #impl_generics grub_split_library::deserialize::Deserialize for #struct_name #ty_generics #where_clause {
            const NUM_BYTES: usize = #num_bytes;
            const ALIGNMENT: usize = #alignment;

            fn deserialize<M: grub_split_library::memory::MemoryReader>(
                reader: &mut M,
                address: grub_split_library::memory::Address,
            ) -> Result<Self, grub_split_library::deserialize::Error> {
                Ok(#create_struct)
            }
        }
    };

    expanded.into()
}

// Add a Deserialize bound to each type parameter
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(
                grub_split_library::deserialize::Deserialize
            ));
        }
    }
    generics
}

// Generate a const expression for NUM_BYTES
fn num_bytes_const(struct_data: &DataStruct) -> TokenStream {
    let mut result = quote!(grub_split_library::memory::Address::new(0));

    for field in &struct_data.fields {
        let ty = &field.ty;
        result = quote_spanned! { field.span() =>
            (#result.align_forward(<#ty>::ALIGNMENT).add_const(<#ty>::NUM_BYTES))
        };
    }
    quote! {
        #result.raw()
    }
}

// Generate a const expression for ALIGNMENT
fn alignment_const(struct_data: &DataStruct) -> TokenStream {
    let mut result = quote!(1);
    for field in &struct_data.fields {
        let ty = &field.ty;
        let alignment = quote_spanned! { field.span() =>
            <#ty>::ALIGNMENT
        };
        result = quote!(max(#result, #alignment));
    }
    quote! {
        {
            const fn max(a: usize, b: usize) -> usize {
                if a >= b { a } else { b }
            }
            #result
        }
    }
}

fn create_struct_expr(struct_data: &DataStruct) -> TokenStream {
    let identifiers: Vec<Ident> = (0..struct_data.fields.len())
        .map(|i| Ident::new(&format!("field{}", i), Span::mixed_site()))
        .collect();

    let initializers = Iterator::zip(struct_data.fields.iter().enumerate(), &identifiers)
        .map(|((i, field), ident)| {
        let ty = &field.ty;
        let extract_field = quote_spanned! { field.span() =>
            next_addr = next_addr.align_forward(<#ty>::ALIGNMENT);
            let #ident = grub_split_library::deserialize::Deserialize::deserialize(
                reader,
                next_addr)?;
        };
        if i == struct_data.fields.len() - 1 {
            extract_field
        } else {
            quote! {
                #extract_field
                next_addr = next_addr + <#ty>::NUM_BYTES;
            }
        }
    });

    let self_arguments =
        Iterator::zip(struct_data.fields.iter().enumerate(), &identifiers).map(
            |((i, field), ident)| {
                if let Some(name) = &field.ident {
                    quote!(#name: #ident)
                } else {
                    let index = Index::from(i);
                    quote!(#index: #ident)
                }
            },
        );

    quote! {
        {
            let mut next_addr = address;
            #(#initializers)*
            Self { #(#self_arguments),* }
        }
    }
}
