use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, ItemImpl, AngleBracketedGenericArguments, PathArguments, GenericArgument};
use proc_macro2::{TokenStream as TokenStream2};
use crate::version::Versions;

pub fn generate_versioned_struct(input: &ItemStruct, versions: &Versions) -> Vec<ItemStruct> {
    let mut structs = Vec::new();
    for version in &versions.0 {
        let mut struct_version = input.clone();
        struct_version.ident = syn::Ident::new(&format!("{}_V{}_{}_{}", input.ident, version.major, version.minor, version.patch), input.ident.span());
        structs.push(struct_version);
    }
    return structs;
}

pub fn generate_versioned_f_like_macros(input: &ItemStruct, versions: &Versions) -> Vec<TokenStream2> {
    let mut f_like_macros = Vec::new();
    for version in &versions.0 {
        let version_ident = format!("{}.{}.{}", version.major, version.minor, version.patch);
        let struct_version = syn::Ident::new(&format!("{}_V{}_{}_{}", input.ident, version.major, version.minor, version.patch), input.ident.span());
        let f_like_macro = quote!([#version_ident] => { #struct_version };);
        f_like_macros.push(f_like_macro);
    }
    return f_like_macros;
}

pub fn generate_versioned_impls(struct_ident: &Option<syn::Ident>, input: &ItemImpl, versions: &Versions) -> Vec<ItemImpl> {
    let mut impls = Vec::new();
    for version in &versions.0 {
        let mut impl_version = input.clone();
        println!("{:?}", impl_version);
        if let syn::Type::Path(path) = &mut *impl_version.self_ty {
            if let Some(ident) = struct_ident {
                if &path.path.segments[0].ident == ident {
                    path.path.segments[0].ident = syn::Ident::new(&format!("{}_V{}_{}_{}", path.path.segments[0].ident, version.major, version.minor, version.patch), path.path.segments[0].ident.span());
                    impls.push(impl_version);
                    continue;
                }
            } else {
                path.path.segments[0].ident = syn::Ident::new(&format!("{}_V{}_{}_{}", path.path.segments[0].ident, version.major, version.minor, version.patch), path.path.segments[0].ident.span());
                impls.push(impl_version);
                continue;
            }
        }

        if let Some(trait_content) = &mut impl_version.trait_ {
            if let PathArguments::AngleBracketed(bracked) = &mut trait_content.1.segments[0].arguments {
                if let GenericArgument::Type(syn::Type::Path(path)) = &mut bracked.args[0] {
                    if let Some(ident) = struct_ident {
                        if &path.path.segments[0].ident == ident {
                            path.path.segments[0].ident = syn::Ident::new(&format!("{}_V{}_{}_{}", path.path.segments[0].ident, version.major, version.minor, version.patch), path.path.segments[0].ident.span());
                            impls.push(impl_version);
                            continue;
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", impls);
    return impls;
}