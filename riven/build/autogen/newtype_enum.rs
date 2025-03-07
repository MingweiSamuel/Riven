use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{parse_quote, File, Ident};

#[derive(Debug, serde::Deserialize)]
pub struct EnumInt {
    pub notes: Option<String>,
    #[serde(rename = "x-value")]
    pub x_value: u32,
    #[serde(rename = "x-deprecated")]
    pub x_deprecated: Option<bool>,
    #[serde(rename = "x-desc")]
    pub x_desc: Option<String>,
    #[serde(rename = "x-name")]
    pub x_name: String,
}

pub fn newtype_enum(variants: &[EnumInt], ident: Ident, repr: Ident, description: &str) -> File {
    let variants = variants.iter().map(
        |EnumInt {
             notes,
             x_value,
             x_deprecated,
             x_desc,
             x_name,
         }| {
            let doc = format!("`{}`", x_value);
            let mut docs = vec![&*doc];
            if let Some(desc) = x_desc {
                docs.extend(desc.lines().map(str::trim));
            }
            if let Some(notes) = notes {
                if !docs.contains(&&**notes) {
                    docs.push("");
                    docs.push(notes);
                }
            }
            let deprecated_attr = x_deprecated.unwrap_or_default().then(|| {
                quote! {
                    #[deprecated(note = #notes)]
                }
            });
            let ident = Ident::new(x_name, Span::call_site());
            let value = Literal::u32_unsuffixed(*x_value);
            quote! {
                #(
                    #[doc = #docs]
                )*
                #deprecated_attr
                #ident = #value,
            }
        },
    );

    parse_quote! {
        #[macro_rules_attribute::apply(newtype_enum)]
        #[repr(#repr)]
        #[doc = #description]
        pub enum #ident {
            #(
                #variants
            )*
        }
    }
}
