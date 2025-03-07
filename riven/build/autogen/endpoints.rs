use std::borrow::Cow;

use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, File, Ident, PatType};

use super::spec::{
    normalize_prop_name, prop_to_type, Operation, Parameter, RequestBody, SchemaEnum, Spec,
};
use super::CODEGEN_NOTE;

pub fn endpoints(spec: &Spec) -> File {
    let mut endpoint_groups = IndexMap::<_, Vec<_>>::new();
    for (path, path_info) in spec.paths.iter() {
        let endpoint = path_info.x_endpoint.as_deref().unwrap();
        endpoint_groups
            .entry(endpoint)
            .or_default()
            .push((path, path_info));
    }

    let impl_riot_api = {
        let methods = endpoint_groups.keys().map(|&endpoint| {
            let method = Ident::new(&endpoint.to_snake_case(), Span::call_site());
            let ty = Ident::new(&endpoint.to_pascal_case(), Span::call_site());
            let doc0 = format!(
                "Returns a handle for accessing [`{}`](crate::endpoints::{}) endpoints.",
                endpoint, ty
            );
            let doc1 = format!(
                "<a href=\"https://developer.riotgames.com/apis#{0}\" target=\"_blank\">`{0}`</a>",
                endpoint
            );
            quote! {
                #[doc = #doc0]
                ///
                /// # Riot Developer API Reference
                #[doc = #doc1]
                ///
                #[doc = #CODEGEN_NOTE]
                #[inline]
                pub fn #method(&self) -> #ty<'_> {
                    #ty { base: self }
                }
            }
        });

        quote! {
            impl RiotApi {
                #( #methods )*
            }
        }
    };

    let endpoint_structs = endpoint_groups.keys().map(|endpoint| {
        let method = Ident::new(&endpoint.to_snake_case(), Span::call_site());
        let ty = Ident::new(&endpoint.to_pascal_case(), Span::call_site());

        let doc0 = format!(
            "{0} endpoints handle, accessed by calling [`{1}()`](crate::RiotApi::{1}) on a [`RiotApi`] instance.",
            endpoint, method,
        );
        let doc1 = format!("<a href=\"https://developer.riotgames.com/apis#{0}\" target=\"_blank\">`{0}`</a>", endpoint);

        quote! {
            #[doc = #doc0]
            ///
            /// # Riot Developer API Reference
            #[doc = #doc1]
            ///
            #[doc = #CODEGEN_NOTE]
            #[repr(transparent)]
            pub struct #ty<'a> {
                base: &'a RiotApi,
            }
        }
    });

    let endpoint_impls = endpoint_groups.iter().map(|(endpoint, methods)| {
        let ty = Ident::new(&endpoint.to_pascal_case(), Span::call_site());

        let methods = methods
            .iter()
            .flat_map(|&(path, path_info)| {
                path_info
                    .iter_operations()
                    .map(move |(verb, op)| (path, verb, op))
            })
            .map(|(path, verb, op)| {
                let op_id = &*op.operation_id;
                let method = Ident::new(&op_id.split_once('.').unwrap().1.to_snake_case(), Span::call_site());

                let ok_resp = &op.responses.ok;
                let is_rso = op
                    .security
                    .as_deref()
                    .is_some_and(|secs| secs.iter().any(|sec| sec.contains_key("rso")));

                // Return type checks.
                let mut parse_type = None;
                let mut return_type = parse_quote!(());
                let mut return_optional = false;
                if let Some(ok_content) = &ok_resp.content {
                    let json_info = ok_content
                        .get("application/json")
                        .expect("Expected JSON response under `application/json` content type.");

                    return_type = prop_to_type(&json_info.schema, false, true);
                    parse_type = Some(return_type.clone());

                    return_optional = op.x_nullable_404.unwrap_or_default();
                    if return_optional {
                        return_type = parse_quote!(Option::<#return_type>);
                    }
                }

                // Rustdoc description.
                let desc_docs = op
                    .description
                    .lines()
                    .map(str::trim);

                let mut method_builder = ArgsBuilder::with_route(op);
                if is_rso {
                    method_builder.add_rso();
                }

                let all_params = op.parameters.as_deref().unwrap_or_default();
                // Path params.
                let mut path_params = all_params
                    .iter()
                    .filter(|p| "path" == p.r#in)
                    .collect::<Vec<_>>();
                // Important: sort path params by their position in the path.
                path_params
                    .sort_unstable_by_key(|p| path.find(&format!("{{{}}}", p.name)).unwrap());
                path_params.iter().for_each(|param| {
                    method_builder.add_param(param);
                });
                // Body param.
                if let Some(body) = &op.request_body {
                    method_builder.add_body(body);
                }
                // Required and optional query params.
                let req_query_params = all_params
                    .iter()
                    .filter(|p| "query" == p.r#in && p.required);
                let opt_query_params = all_params
                    .iter()
                    .filter(|p| "query" == p.r#in && !p.required);
                req_query_params.into_iter().chain(opt_query_params).for_each(|param| {
                    method_builder.add_param(param);
                });
                // Header params.
                if all_params.iter().any(|p| "header" == p.r#in) {
                    panic!("Header params are not implemented: {:?}", all_params);
                }

                let path_argument = path_argument(path, path_params);

                let rso_docs = is_rso.then(|| quote! {
                    /// # RSO
                    /// This endpoint uses [Riot Sign On](https://developer.riotgames.com/docs/lol#rso-integration)
                    /// via the `access_token` parameter, instead of the Riot API key.
                    ///
                });
                let doc_link = format!("<a href=\"{}\" target=\"_blank\">`{}`</a>", op.external_docs.url, op_id);

                let ArgsBuilder { arg_docs, arg_inputs, arg_code } = method_builder;

                let verb_ident = Ident::new(&verb.to_ascii_uppercase(), Span::call_site());

                let exec_fn = if ok_resp.content.is_some() {
                    if return_optional {
                        quote! { execute_opt }
                    } else {
                        quote! { execute_val }
                    }
                } else {
                    quote! { execute }
                };

                quote! {
                    #( #[doc = #desc_docs] )*
                    ///
                    /// # Parameters
                    #( #[doc = #arg_docs] )*
                    ///
                    #rso_docs
                    /// # Riot Developer API Reference
                    #[doc = #doc_link]
                    ///
                    #[doc = #CODEGEN_NOTE]
                    pub fn #method(&self, #( #arg_inputs ),*) -> impl 'a + Future<Output = Result<#return_type>>
                    {
                        let route_str = route.into();
                        let request = self.base.request(Method::#verb_ident, route_str, #path_argument);
                        #( #arg_code )*
                        let future = self.base.#exec_fn::<#parse_type>(#op_id, route_str, request);
                        #[cfg(feature = "tracing")]
                        let future = future.instrument(tracing::info_span!(#op_id, route = route_str));
                        #[cfg(feature = "metrics")]
                        let future = metrics::timed(future, #op_id, route_str);
                        future
                    }
                }
            });

        quote! {
            #[doc = #CODEGEN_NOTE]
            impl<'a> #ty<'a> {
                #( #methods )*
            }
        }
    });

    parse_quote! {
        use std::future::Future;
        use std::vec::Vec;

        #[cfg(feature="metrics")]
        use crate::metrics;

        #[cfg(feature="tracing")]
        use tracing::Instrument;
        use reqwest::Method;

        use crate::Result;
        use crate::consts::{ RegionalRoute, PlatformRoute, ValPlatformRoute };
        use crate::riot_api::RiotApi;

        #impl_riot_api

        #( #endpoint_structs )*

        #( #endpoint_impls )*
    }
}

#[derive(Default)]
pub struct ArgsBuilder {
    pub arg_docs: Vec<Cow<'static, str>>,
    pub arg_inputs: Vec<PatType>,
    pub arg_code: Vec<TokenStream>,
}
impl ArgsBuilder {
    pub fn with_route(op: &Operation) -> Self {
        let mut this = Self::default();
        this.arg_inputs.push({
            let route_ty = Ident::new(
                &format!("{}Route", op.x_route_enum.to_pascal_case()),
                Span::call_site(),
            );
            parse_quote!(route: #route_ty)
        });
        this.arg_docs.push("- `route` - Route to query.".into());
        this
    }

    pub fn add_rso(&mut self) {
        self.arg_docs
            .push("- `access_token` - RSO access token.".into());
        self.arg_inputs
            .push(parse_quote!(access_token: impl std::fmt::Display));
        self.arg_code.push(quote! {
            let mut request = request.bearer_auth(access_token);
            if let Some(clear) = self.base.get_rso_clear_header() {
                request = request.header(clear, "")
            }
        });
    }

    pub fn add_body(&mut self, body: &RequestBody) {
        let name = &parse_quote!(body);
        let json_info = body.content.get("application/json").unwrap();
        let ty = prop_to_type(&json_info.schema, !body.required, false);
        // Docs.
        self.push_doc_line("body", body.required, "body", &body.description);
        // Args.
        self.arg_inputs.push(parse_quote!(#name: &#ty));
        // Code.
        let body_code = quote! {
            request
                .body(serde_json::ser::to_vec(#name).unwrap())
                .header(reqwest::header::CONTENT_TYPE, "application/json")
        };
        self.push_code_optional(name, body_code, body.required);
    }

    pub fn add_param(&mut self, param: &Parameter) {
        let required = param.required || "path" == param.r#in;
        let key = &*param.name;
        let name = &normalize_prop_name(key, None);
        let ty = prop_to_type(&param.schema, !required, false);
        // Docs.
        self.push_doc_line(name, required, &param.r#in, &param.description);
        // Args.
        self.arg_inputs.push(parse_quote!(#name: #ty));
        // Code (path params are handled separately).
        if "path" != param.r#in {
            let param_code = match &param.schema.schema_enum() {
                Some(SchemaEnum::Array { .. }) => quote! {
                    request.query(&*#name
                        .iter()
                        .map(|w| (#key, w))
                        .collect::<::std::vec::Vec<_>>())
                },
                Some(SchemaEnum::Object { .. }) => {
                    panic!("object not expected in query param: `{:?}`.", param.name)
                }
                Some(_other) => quote! {
                    request.query(&[(#key, #name)])
                },
                None => panic!("schema not expected in query param: `{:?}`.", param.name),
            };
            self.push_code_optional(name, param_code, required);
        }
    }

    fn push_code_optional(&mut self, name: &Ident, mut code: TokenStream, required: bool) {
        if !required {
            code = quote! {
                if let Some(#name) = #name {
                    #code
                } else {
                    request
                }
            };
        }
        self.arg_code.push(quote! {
            let request = #code;
        });
    }

    fn push_doc_line(
        &mut self,
        name: impl std::fmt::Display,
        required: bool,
        r#in: &str,
        description: &Option<String>,
    ) {
        let mut doc_line = format!(
            "- `{}` ({}, in {})",
            name,
            if required { "required" } else { "optional" },
            r#in
        );
        if let Some(description) = description {
            doc_line.push_str(" - ");
            doc_line.push_str(description);
            if !description.ends_with(['.', '!', '?']) {
                doc_line.push('.');
            }
        }
        self.arg_docs.push(doc_line.into());
    }
}

fn path_argument(path: &String, path_params: Vec<&Parameter>) -> TokenStream {
    if path_params.is_empty() {
        quote!(#path)
    } else {
        // Replaces `{someName}` with `{}` for the format string.
        let path = path
            .chars()
            .filter({
                let mut active = true;
                move |c| match c {
                    '{' => {
                        active = false;
                        true
                    }
                    '}' => {
                        active = true;
                        true
                    }
                    _ => active,
                }
            })
            .collect::<String>();
        let path_param_idents = path_params
            .iter()
            .map(|p| normalize_prop_name(&p.name, None));
        quote! {
            &format!(
                #path,
                #( #path_param_idents ),*
            )
        }
    }
}
