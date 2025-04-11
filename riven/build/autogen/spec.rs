#![allow(dead_code)]

use std::collections::HashSet;

use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use proc_macro2::Span;
use serde_json::Value;
use syn::{parse_quote, Ident, Type};

#[derive(Debug, serde::Deserialize)]
pub struct Spec {
    pub openapi: String,
    pub info: Info,
    // pub servers: Vec<Server>,
    pub paths: IndexMap<String, Path>,
    pub components: Components,
}

#[derive(Debug, serde::Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Path {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub delete: Option<Operation>,
    pub patch: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    #[serde(rename = "x-endpoint")]
    pub x_endpoint: Option<String>,
    #[serde(rename = "x-platforms-available")]
    pub x_platforms_available: Option<Vec<String>>,
    #[serde(rename = "x-route-enum")]
    pub x_route_enum: Option<String>,
}
impl Path {
    pub fn iter_operations(&self) -> impl Iterator<Item = (&'static str, &Operation)> {
        IntoIterator::into_iter([
            ("get", &self.get),
            ("post", &self.post),
            ("put", &self.put),
            ("delete", &self.delete),
            ("patch", &self.patch),
            ("options", &self.options),
            ("head", &self.head),
        ])
        .filter_map(|(method, op)| op.as_ref().map(|op| (method, op)))
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Operation {
    pub tags: Vec<String>,
    pub summary: String,
    pub description: String,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub parameters: Option<Vec<Parameter>>,
    #[serde(rename = "externalDocs")]
    pub external_docs: ExternalDocs,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,
    pub responses: Responses,
    pub security: Option<Vec<IndexMap<String, Vec<String>>>>,
    #[serde(rename = "x-route-enum")]
    pub x_route_enum: String,
    #[serde(rename = "x-nullable-404")]
    pub x_nullable_404: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Parameter {
    pub name: String,
    pub r#in: String,
    pub schema: SchemaOrRef,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestBody {
    pub content: IndexMap<String, MediaType>,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Responses {
    #[serde(rename = "200")]
    pub ok: Response,
}

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    pub description: String,
    pub content: Option<IndexMap<String, MediaType>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MediaType {
    pub schema: SchemaOrRef,
    pub encoding: Option<IndexMap<String, Value>>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum SchemaOrRef {
    Schema(Schema),
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
        #[serde(rename = "x-type")]
        x_type: Option<String>,
    },
}
impl SchemaOrRef {
    pub fn schema_enum(&self) -> Option<&SchemaEnum> {
        match self {
            SchemaOrRef::Schema(Schema {
                schema: schema_enum,
                ..
            }) => Some(schema_enum),
            _ => None,
        }
    }
    pub fn description(&self) -> Option<&str> {
        match self {
            SchemaOrRef::Schema(schema_real) => schema_real.description.as_deref(),
            _ => None,
        }
    }
    pub fn x_alias(&self) -> Option<&str> {
        match self {
            SchemaOrRef::Schema(schema_real) => schema_real.x_alias.as_deref(),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Schema {
    #[serde(flatten)]
    pub schema: SchemaEnum,
    pub description: Option<String>,
    #[serde(rename = "x-type")]
    pub x_type: Option<String>,
    #[serde(rename = "x-alias")]
    pub x_alias: Option<String>,
}
impl Schema {
    fn x_enum(&self) -> Option<&str> {
        match &self.schema {
            SchemaEnum::String { x_enum, .. } => x_enum.as_deref(),
            SchemaEnum::Integer { x_enum, .. } => x_enum.as_deref(),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum SchemaEnum {
    #[serde(rename = "array")]
    Array {
        #[serde(rename = "items")]
        items: Box<SchemaOrRef>,
    },
    #[serde(rename = "object")]
    Object {
        title: Option<String>,
        #[serde(rename = "properties")]
        properties: Option<IndexMap<String, SchemaOrRef>>,
        #[serde(rename = "required")]
        required: Option<Vec<String>>,
        #[serde(rename = "additionalProperties")]
        additional_properties: Option<Box<SchemaOrRef>>,
        #[serde(rename = "x-key")]
        x_key: Option<Box<SchemaOrRef>>,
    },
    #[serde(rename = "string")]
    String {
        #[serde(rename = "enum")]
        r#enum: Option<Vec<String>>,
        #[serde(rename = "x-enum")]
        x_enum: Option<String>,
    },
    #[serde(rename = "integer")]
    Integer {
        format: Option<String>,
        #[serde(rename = "x-enum")]
        x_enum: Option<String>,
    },
    #[serde(rename = "number")]
    Number { format: String },
    #[serde(rename = "boolean")]
    Boolean,
}

#[derive(Debug, serde::Deserialize)]
pub struct ExternalDocs {
    pub description: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Components {
    pub schemas: IndexMap<String, SchemaOrRef>,
    // #[serde(rename = "securitySchemes")]
    // security_schemes: IndexMap<String, SecurityScheme>,
}

pub fn normalize_schema_name(schema_name: &str) -> Ident {
    let schema_name = schema_name.replace("DTO", "").replace("Dto", "");
    Ident::new(&schema_name, Span::call_site())
}

pub fn normalize_prop_name(prop_name: &str, dedup_names: Option<&mut HashSet<String>>) -> Ident {
    let mut prop_name = prop_name.to_snake_case();
    if prop_name.starts_with(|c| char::is_ascii_digit(&c)) {
        prop_name = format!("x{}", prop_name);
    }

    if let Some(names) = dedup_names {
        while names.contains(&prop_name) {
            prop_name.push('_');
        }
        names.insert(prop_name.clone());
    }

    if "type" == prop_name {
        Ident::new_raw(&prop_name, Span::call_site())
    } else {
        Ident::new(&prop_name, Span::call_site())
    }
}

pub fn prop_to_type(prop: &SchemaOrRef, optional: bool, owned: bool) -> Type {
    if optional {
        let prop = prop_to_type(prop, false, owned);
        return parse_quote!(Option<#prop>);
    }
    let schema = match prop {
        SchemaOrRef::Schema(schema) => schema,
        SchemaOrRef::Ref { r#ref, .. } => {
            let (endpoint, schema) = r#ref
                .strip_prefix("#/components/schemas/")
                .unwrap()
                .split_once('.')
                .unwrap();
            let endpoint = Ident::new(&endpoint.to_snake_case(), Span::call_site());
            let schema = normalize_schema_name(schema);
            return parse_quote!(crate::models::#endpoint::#schema);
        }
    };

    if let Some(x_enum) = schema.x_enum() {
        if "locale" != x_enum {
            let enum_ident = Ident::new(&x_enum.to_pascal_case(), Span::call_site());
            return parse_quote!(crate::consts::#enum_ident);
        };
    }
    match &schema.schema {
        SchemaEnum::Array { items } => {
            let subtype = prop_to_type(items, false, owned);
            if owned {
                parse_quote!(Vec<#subtype>)
            } else {
                parse_quote!(&[#subtype])
            }
        }
        SchemaEnum::Object {
            title: _,
            properties,
            required,
            additional_properties,
            x_key,
        } => {
            assert!(
                properties.is_none(),
                "Schema for is nested object, cannot turn into type name",
            );
            assert!(required.is_none());

            if let (Some(keys), Some(values)) = (x_key, additional_properties) {
                let key_type = prop_to_type(keys, false, owned);
                let value_type = prop_to_type(values, false, owned);
                parse_quote!(::std::collections::HashMap::<#key_type, #value_type>)
            } else {
                // Only `{ "type": "object" }`.
                parse_quote!(crate::serde_json::Map::<String, crate::serde_json::Value>)
            }
        }
        SchemaEnum::String { .. } => {
            if owned {
                parse_quote!(String)
            } else {
                parse_quote!(&str)
            }
        }
        SchemaEnum::Integer { format, .. } => match format.as_deref().unwrap() {
            "int32" => parse_quote!(i32),
            "int64" => parse_quote!(i64),
            unknown => panic!("Unknown integer format: {}", unknown),
        },
        SchemaEnum::Number { format } => match &**format {
            "float" => parse_quote!(f32),
            "double" => parse_quote!(f64),
            unknown => panic!("Unknown number format: {}", unknown),
        },
        SchemaEnum::Boolean => parse_quote!(bool),
    }
}
