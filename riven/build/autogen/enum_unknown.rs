use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, File, Ident};

#[derive(Debug, serde::Deserialize)]
pub struct EnumString {
    #[serde(rename = "x-name")]
    pub x_name: String,
    #[serde(rename = "x-desc")]
    pub x_desc: String,
    #[serde(rename = "x-deprecated")]
    pub x_deprecated: Option<bool>,
    pub notes: Option<String>,
}

/// Enum with `UNKNOWN(String)` variant for new/unknown values.
pub fn enum_unknown(variants: &[EnumString], ident: Ident, repr: Ident, description: &str) -> File {
    let variants = variants.iter().map(
        |EnumString {
             x_name,
             x_desc,
             x_deprecated,
             notes,
         }| {
            let docs = x_desc.lines().map(str::trim);
            let notes_attr = notes.as_deref().map(|notes| {
                quote! {
                    ///
                    #[doc = #notes]
                }
            });
            let deprecated_attr = x_deprecated.unwrap_or_default().then(|| {
                let notes = notes.as_deref().map(|notes| quote!(note = #notes));
                quote!(#[deprecated(#notes)])
            });
            let key = Ident::new(x_name, Span::call_site());
            quote! {
                #( #[doc = #docs] )*
                #notes_attr
                #deprecated_attr
                #key,
            }
        },
    );

    parse_quote! {
        use strum_macros::{ EnumString, EnumVariantNames, IntoStaticStr };

        #[doc = #description]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[derive(EnumString, EnumVariantNames, IntoStaticStr)]
        #[repr(#repr)]
        pub enum #ident {
            /// Catch-all variant for new/unknown values.
            #[strum(default)]
            UNKNOWN(String),

            #( #variants )*
        }

        serde_strum_unknown!(#ident);
    }
}
