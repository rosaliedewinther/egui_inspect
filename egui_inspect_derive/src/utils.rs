use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::Type::{Path, Reference};
use syn::{Field, Type};

use crate::AttributeArgs;

pub fn get_path_str(type_path: &Type) -> Option<String> {
    match type_path {
        Path(type_path) => {
            let ident = type_path
                .path
                .get_ident();
            if let Some(name) = ident {
                return Some(name.to_string());
            }
            None
        }
        Reference(type_ref) => get_path_str(&type_ref.elem),
        _ => Some("".to_string()),
    }
}

pub(crate) fn get_default_function_call(field: &Field, mutable: bool, attrs: &AttributeArgs) -> TokenStream {
    let name = &field.ident;

    let name_str = match &attrs.name {
        Some(n) => n.clone(),
        None => name.clone().unwrap().to_string(),
    };

    if mutable {
        quote_spanned! {field.span() => {egui_inspect::EguiInspect::inspect_mut(&mut self.#name, &#name_str, ui);}}
    } else {
        quote_spanned! {field.span() => {egui_inspect::EguiInspect::inspect(&self.#name, &#name_str, ui);}}
    }
}
