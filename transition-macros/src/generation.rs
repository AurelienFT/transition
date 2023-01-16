use crate::version::{Version, Versions};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    visit_mut::{self, VisitMut},
    ItemImpl, ItemStruct, Type,
};

//fn filter_fields(&mut )

pub fn generate_versioned_struct(input: &ItemStruct, versions: &Versions) -> Vec<ItemStruct> {
    let mut structs = Vec::new();
    for version in &versions.0 {
        let mut struct_version = input.clone();
        struct_version.ident = syn::Ident::new(
            &format!(
                "{}V{}_{}_{}",
                input.ident, version.major, version.minor, version.patch
            ),
            input.ident.span(),
        );
        //filter_fields(&mut struct_version, version);
        structs.push(struct_version);
    }
    return structs;
}

pub fn generate_versioned_f_like_macros(
    input: &ItemStruct,
    versions: &Versions,
) -> Vec<TokenStream2> {
    let mut f_like_macros = Vec::new();
    for version in &versions.0 {
        let version_ident = format!("{}.{}.{}", version.major, version.minor, version.patch);
        let struct_version = syn::Ident::new(
            &format!(
                "{}V{}_{}_{}",
                input.ident, version.major, version.minor, version.patch
            ),
            input.ident.span(),
        );
        let f_like_macro = quote!([#version_ident] => { #struct_version };);
        f_like_macros.push(f_like_macro);
    }
    return f_like_macros;
}

struct ImplVisitor<'a> {
    version: &'a Version,
    struct_ident: &'a syn::Ident,
}

impl<'a> VisitMut for ImplVisitor<'a> {
    fn visit_type_mut(&mut self, node: &mut Type) {
        // Sub-recurse (not really needed here since there aren't
        // sub-expressions within an ExprType):
        if let Type::Path(path) = node {
            if self.struct_ident == &path.path.segments[0].ident {
                path.path.segments[0].ident = syn::Ident::new(
                    &format!(
                        "{}V{}_{}_{}",
                        path.path.segments[0].ident,
                        self.version.major,
                        self.version.minor,
                        self.version.patch
                    ),
                    path.path.segments[0].ident.span(),
                );
            }
        }
        visit_mut::visit_type_mut(self, node);
    }
}

pub fn generate_versioned_impls(
    struct_ident: &Option<syn::Ident>,
    input: &ItemImpl,
    versions: &Versions,
) -> Vec<ItemImpl> {
    let mut impls = Vec::new();
    for version in &versions.0 {
        let mut impl_version = input.clone();
        if let Some(struct_ident) = struct_ident {
            let mut visitor = ImplVisitor {
                version,
                struct_ident,
            };
            visitor.visit_item_impl_mut(&mut impl_version);
            impls.push(impl_version);
        } else {
            if let syn::Type::Path(path) = &mut *impl_version.self_ty {
                path.path.segments[0].ident = syn::Ident::new(
                    &format!(
                        "{}V{}_{}_{}",
                        path.path.segments[0].ident, version.major, version.minor, version.patch
                    ),
                    path.path.segments[0].ident.span(),
                );
                impls.push(impl_version);
                continue;
            }
        }
    }
    return impls;
}
