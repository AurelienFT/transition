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
    let mut args = Args::from_list(&attr_args).unwrap();
    args.versions.0.sort();
    let mut struct_versioned = parse_macro_input!(input as ItemStruct);
    struct_versioned.attrs.retain(|attr| attr.path.is_ident("transition"));
    let highest_version = args.versions.0.last().unwrap().to_ident(&struct_versioned.ident);
    let ident = struct_versioned.ident.clone();
    let default_type = quote::quote! {
        type #ident = #highest_version;
    };
    let structs = generate_versioned_struct(&struct_versioned, &args.versions);
    let struct_name = struct_versioned.ident.clone();
    let f_like_macros = generate_versioned_f_like_macros(&struct_versioned, &args.versions);
    TokenStream::from(quote::quote! {
        #default_type

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
    let impl_versioned = parse_macro_input!(input as ItemImpl);
    let impls = generate_versioned_impls(&args.structure, &impl_versioned, &args.versions);
    TokenStream::from(quote::quote! {
        #(#impls)*
    })
}

// Placeholder for field
#[proc_macro_attribute]
pub fn field(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}