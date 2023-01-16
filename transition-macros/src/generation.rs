use crate::{version::{Version, Versions}, Args};
use darling::{FromMeta, util::parse_attribute_to_meta_list};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    visit_mut::{self, VisitMut},
    Fields,
    ItemImpl, ItemStruct, Type, NestedMeta, Field,
};

fn filter_fields(struct_version: &mut ItemStruct, version: &Version) {
    if let Fields::Named(fields) = &mut struct_version.fields {
        fields.named = fields.named.iter().filter_map(|field: &Field| {
            let mut final_field = Some(field.clone());
            let final_attrs = field.attrs.iter().filter_map(|attr| {
                if attr.path.segments.len() > 1 {
                    if attr.path.segments[0].ident == "transition" && attr.path.segments[1].ident == "field" {
                        let attr_args = parse_attribute_to_meta_list(attr).unwrap();
                        let args = Args::from_list(&attr_args.nested.into_iter().collect::<Vec<NestedMeta>>()).unwrap();
                        if !args.versions.0.contains(version) {
                            final_field = None;
                        }
                        None
                    } else {
                        Some(attr.clone())
                    }
                } else {
                    Some(attr.clone())
                }
            }).collect();
            if let Some(f) = &mut final_field {
                f.attrs = final_attrs;
            }
            final_field
        }).collect();
    }
}

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
        filter_fields(&mut struct_version, version);
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
            if let Some(ident) = path.path.get_ident() {
                if self.struct_ident == ident {
                    path.path.segments[0].ident = syn::Ident::new(
                        &format!(
                            "{}V{}_{}_{}",
                            ident,
                            self.version.major,
                            self.version.minor,
                            self.version.patch
                        ),
                        ident.span(),
                    );
                }
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
                if let Some(ident) = path.path.get_ident() {
                    path.path.segments[0].ident = syn::Ident::new(
                        &format!(
                            "{}V{}_{}_{}",
                            ident, version.major, version.minor, version.patch
                        ),
                        ident.span(),
                    );
                    impls.push(impl_version);
                    continue;
                }
            }
        }
    }
    return impls;
}
