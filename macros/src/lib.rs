use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput};

fn parse_bundle(attr: &Attribute) -> TokenStream {
    if attr.tokens.is_empty() {
        TokenStream::from(quote! { () })
    } else {
        let mut str = attr.tokens.to_string();
        // This handles the trailing ',' needed to crate a bundle tuple from a single component:
        if str != "()" && str.ends_with(")") && !str.contains(',') {
            str.insert(str.len() - 1, ',');
        }
        str.parse().unwrap()
    }
}

#[proc_macro_derive(EntityKind, attributes(defaults, components))]
pub fn derive_entity_kind(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let ident = input.ident;

    let defaults = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("defaults"))
        .map(|components_attr: &Attribute| parse_bundle(components_attr))
        .unwrap_or_else(|| TokenStream::from(quote! { () }));

    let components = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("components"))
        .map(|components_attr: &Attribute| parse_bundle(components_attr))
        .unwrap_or_else(|| TokenStream::from(quote! { () }));

    proc_macro::TokenStream::from(quote! {
        impl bevy_kindly::EntityKind for #ident {
            type DefaultBundle = #defaults;

            type Bundle = #components;

            unsafe fn from_entity_unchecked(entity: Entity) -> Self {
                Self(entity)
            }

            fn entity(&self) -> Entity {
                self.0
            }
        }
    })
}
