use std::borrow::Cow;
use std::collections::HashSet;

use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, File, Ident};

use super::spec::{
    normalize_prop_name, normalize_schema_name, prop_to_type, Schema, SchemaEnum, SchemaOrRef, Spec,
};
use super::{process_description, CODEGEN_NOTE};

pub fn models(spec: &Spec) -> File {
    let mut schemas_by_endpoint = IndexMap::<_, Vec<_>>::new();
    for (schema_key, schema) in spec.components.schemas.iter() {
        if "Error" == *schema_key {
            continue;
        }
        let endpoint = schema_key.split_once('.').unwrap().0;
        schemas_by_endpoint
            .entry(endpoint)
            .or_default()
            .push((schema_key, schema));
    }

    let endpoint_modules = schemas_by_endpoint.into_iter().map(|(endpoint, schemas)| {
        let endpoint_ident = Ident::new(&endpoint.to_snake_case(), Span::call_site());
        let doc = format!(
            "Data structs used by [`{0}`](crate::endpoints::{0})",
            endpoint.to_pascal_case()
        );

        let schemas = schemas
            .into_iter()
            .map::<syn::ItemStruct, _>(|(schema_key, schema)| {
                let schema_name_raw = schema_key.split_once('.').unwrap().1;
                let schema_ident = normalize_schema_name(schema_name_raw);
                let SchemaOrRef::Schema(Schema {
                    schema:
                        SchemaEnum::Object {
                            title: _,
                            properties,
                            required,
                            additional_properties: _,
                            x_key: _,
                        },
                    description,
                    x_type: _,
                    x_alias: _,
                }) = &schema
                else {
                    panic!("Schema for `{}` is not an object: {:?}", schema_key, schema);
                };
                let required = required.iter().flatten().collect::<HashSet<_>>();

                // Field docs
                let mut docs = vec![Cow::Owned(format!("`{}.{}` data object.", endpoint, schema_name_raw))];
                if let Some(description) = description {
                    docs.push("".into());
                    docs.push("# Description".into());
                    docs.extend(process_description(description));
                }

                // Generate the fields
                let mut dedup_names = HashSet::new();
                let fields = properties.iter().flatten().map(|(prop_key, prop)| {
                    let optional = !required.contains(prop_key);
                    let prop_name = normalize_prop_name(prop_key, Some(&mut dedup_names));

                    // Docs
                    let mut docs = prop
                        .description()
                        .into_iter()
                        .flat_map(process_description)
                        .collect::<Vec<_>>();

                    // Serde attributes
                    let mut serde_metas = vec![quote!(rename = #prop_key)];
                    if let Some(alias) = prop.x_alias() {
                        serde_metas.push(quote!(alias = #alias));
                    }
                    if optional {
                        serde_metas.push(quote!(default));
                        serde_metas.push(quote!(skip_serializing_if = "Option::is_none"));
                    }

                    let mut deprecated = None;
                    let mut ty = prop_to_type(prop, optional, true);

                    // Special handling for specific fields
                    if "championId" == prop_key && prop.description().is_some_and(|d| d.contains("this field returned invalid championIds")) {
                        docs.push("".into());
                        docs.push("Instead use [`Self::champion()`] which checks this field then parses [`Self::champion_name`].".into());
                        docs.push("<https://github.com/RiotGames/developer-relations/issues/553>".into());
                        deprecated = Some(quote!(#[deprecated(since = "2.5.0", note = "Use `Participant.champion()` instead. Riot sometimes returns corrupted data for this field: https://github.com/RiotGames/developer-relations/issues/553")]));
                        serde_metas.push(quote!(serialize_with = "crate::consts::Champion::serialize_result"));
                        serde_metas.push(quote!(deserialize_with = "crate::consts::Champion::deserialize_result"));
                        ty = parse_quote!(Result<crate::consts::Champion, std::num::TryFromIntError>);
                    }
                    else if "gameType" == prop_key && "InfoDto" == schema_name_raw && "match-v5" == endpoint {
                        docs.push("".into());
                        docs.push("Will be `None` if empty string is returned: <https://github.com/RiotGames/developer-relations/issues/898>".into());
                        serde_metas.push(quote!(serialize_with = "crate::consts::serialize_empty_string_none"));
                        serde_metas.push(quote!(deserialize_with = "crate::consts::deserialize_empty_string_none"));
                        ty = parse_quote!(Option<#ty>);
                    }

                    quote! {
                        #( #[doc = #docs] )*
                        #deprecated
                        #[serde( #( #serde_metas ),* )]
                        pub #prop_name: #ty,
                    }
                });

                parse_quote! {
                    #( #[doc = #docs] )*
                    ///
                    #[doc = #CODEGEN_NOTE]
                    #[derive(Clone, Debug)]
                    #[derive(serde::Serialize, crate::de::Deserialize)]
                    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
                    pub struct #schema_ident {
                        #( #fields )*
                    }
                }
            });

        quote! {
            #[doc = #doc]
            ///
            #[doc = #CODEGEN_NOTE]
            #[allow(dead_code)]
            pub mod #endpoint_ident {
                #( #schemas )*
            }
        }
    });

    parse_quote! {
        #( #endpoint_modules )*
    }
}
