use syn::{parse_quote, File};

use super::spec::Spec;

pub fn meta(spec: &Spec) -> File {
    let rows = spec
        .paths
        .iter()
        .flat_map(|(route, path)| {
            path.iter_operations()
                .map(move |(method, operation)| (&**route, method, operation))
        })
        .map(|(route, method, operation)| {
            let method_ident =
                syn::Ident::new(&method.to_ascii_uppercase(), proc_macro2::Span::call_site());
            let operation_id = &operation.operation_id;
            quote::quote! {
                (Method::#method_ident, #route, #operation_id),
            }
        })
        .collect::<Vec<_>>();
    let len = rows.len();

    parse_quote! {
        use reqwest::Method;

        /// Metadata for endpoints. Each tuple corresponds to one endpoint and contains
        /// the HTTP [`Method`], `str` path, and the method's `str` ID.
        pub static ALL_ENDPOINTS: [(Method, &str, &str); #len] = [
            #( #rows )*
        ];
    }
}
