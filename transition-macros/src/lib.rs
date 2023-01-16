use generation::{generate_versioned_impls, generate_versioned_f_like_macros};
use proc_macro::TokenStream;
use darling::FromMeta;
use syn::{parse_macro_input, AttributeArgs, ItemStruct, ItemImpl};
use version::Versions;

use crate::generation::generate_versioned_struct;

mod generation;
mod version;

#[derive(Debug, FromMeta)]
struct Args {
    versions: Versions,
}

#[derive(Debug, FromMeta)]
struct ArgsImpl {
    versions: Versions,
    structure: Option<syn::Ident>
}

#[proc_macro_attribute]
pub fn versioned(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let args = Args::from_list(&attr_args).unwrap();
    let mut struct_versioned = parse_macro_input!(input as ItemStruct);
    //TODO: Remove only transition ones
    struct_versioned.attrs.retain(|attr| attr.path.is_ident("transition"));
    let default_struct = quote::quote! {
        #struct_versioned
    };
    let structs = generate_versioned_struct(&struct_versioned, &args.versions);
    let struct_name = struct_versioned.ident.clone();
    let f_like_macros = generate_versioned_f_like_macros(&struct_versioned, &args.versions);
    TokenStream::from(quote::quote! {
        #default_struct

        macro_rules! #struct_name {
            #(#f_like_macros)*
        }

        #(#structs)*
    })
}

#[proc_macro_attribute]
pub fn impl_version(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let args = ArgsImpl::from_list(&attr_args).unwrap();
    let mut impl_versioned = parse_macro_input!(input as ItemImpl);
    println!("{:?}", args);
    let impls = generate_versioned_impls(&args.structure, &impl_versioned, &args.versions);
    TokenStream::from(quote::quote! {
        #(#impls)*
    })
}