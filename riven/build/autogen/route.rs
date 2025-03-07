use indexmap::IndexMap;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{parse_quote, File, Ident, ItemEnum};

#[derive(Debug, serde::Deserialize)]
pub struct RoutesTable {
    pub regional: IndexMap<String, Route>,
    pub platform: IndexMap<String, Route>,
    #[serde(rename = "val-platform")]
    pub val_platform: IndexMap<String, Route>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Route {
    pub id: u8,
    pub description: String,
    pub deprecated: Option<bool>,
    #[serde(rename = "altName")]
    pub alt_name: Option<String>,
    #[serde(rename = "tournamentRegion")]
    pub tournament_region: Option<String>,
    #[serde(rename = "regionalRoute")]
    pub regional_route: Option<String>,
    #[serde(rename = "regionalRouteLor")]
    pub regional_route_lor: Option<String>,
}

pub fn make_enum(routes: &IndexMap<String, Route>, ident: Ident, description: &str) -> ItemEnum {
    let variants = routes.iter().map(
        |(
            route_name,
            Route {
                id,
                description,
                deprecated,
                alt_name,
                ..
            },
        )| {
            let route_name = route_name.to_ascii_uppercase();
            let docs = description.lines().map(str::trim);
            let doc_repr = format!("`{}` (riotapi-schema ID/repr)", id);
            let deprecated_attr = deprecated
                .unwrap_or_default()
                .then(|| quote!(#[deprecated]));
            let serialize = alt_name
                .as_deref()
                .filter(|&alt_name| route_name != alt_name)
                // Note: strum(serialize = ...) actually specifies extra **De**serialization values.
                .map(|alt_name| quote!(serialize = #alt_name));
            let key = Ident::new(&route_name, Span::call_site());
            let value = Literal::u8_unsuffixed(*id);

            quote! {
                #( #[doc = #docs] )*
                ///
                #[doc = #doc_repr]
                #deprecated_attr
                #[strum(to_string=#route_name, #serialize)]
                #key = #value,
            }
        },
    );
    parse_quote! {
        #[doc = #description]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[derive(IntoPrimitive, TryFromPrimitive)]
        #[derive(EnumString, EnumIter, Display, IntoStaticStr)]
        #[repr(u8)]
        #[non_exhaustive]
        pub enum #ident {
            #(
                #variants
            )*
        }
    }
}

pub fn route(routes_table: &RoutesTable) -> File {
    let regional_route = make_enum(
        &routes_table.regional,
        parse_quote!(RegionalRoute),
        "Regional routes, used in tournament services, Legends of Runeterra (LoR), and some other endpoints.",
    );

    let platform_route = make_enum(
        &routes_table.platform,
        parse_quote!(PlatformRoute),
        "Platform routes for League of Legends (LoL), Teamfight Tactics (TFT), and Legends of Runeterra (LoR).",
    );
    let platform_route_impl = {
        let route_idents = routes_table
            .platform
            .keys()
            .map(|route_name| Ident::new(&route_name.to_ascii_uppercase(), Span::call_site()))
            .collect::<Vec<_>>();

        let regional_idents = routes_table.platform.values().map(|route| {
            Ident::new(
                &route
                    .regional_route
                    .as_deref()
                    .unwrap()
                    .to_ascii_uppercase(),
                Span::call_site(),
            )
        });

        let lor_idents = routes_table.platform.values().map(|route| {
            Ident::new(
                &route
                    .regional_route_lor
                    .as_deref()
                    .unwrap()
                    .to_ascii_uppercase(),
                Span::call_site(),
            )
        });

        let tournament_regions = routes_table.platform.values().map(|route| {
            route
                .tournament_region
                .as_deref()
                .map(|tournament_region| {
                    let ident = Ident::new(tournament_region, Span::call_site());
                    quote!(Some(TournamentRegion::#ident))
                })
                .unwrap_or(quote!(None))
        });

        let alt_names = routes_table.platform.iter().map(|(route_name, route)| {
            route
                .alt_name
                .as_deref()
                .map(|alt_name| quote!(#alt_name))
                .unwrap_or_else(|| {
                    let route_name = route_name.to_ascii_uppercase();
                    quote!(#route_name)
                })
        });

        quote! {
            impl PlatformRoute {
                /// Converts this [`PlatformRoute`] into its corresponding
                /// [`RegionalRoute`] for LoL and TFT match endpoints such as
                /// [`match-v5`](crate::endpoints::MatchV5).
                pub fn to_regional(self) -> RegionalRoute {
                    match self {
                        #(
                            Self::#route_idents => RegionalRoute::#regional_idents,
                        )*
                    }
                }

                /// Converts this [`PlatformRoute`] into its corresponding
                /// [`RegionalRoute`] for LoR endpoints such as
                /// [`lor-match-v1`](crate::endpoints::LorMatchV1).
                pub fn to_regional_lor(self) -> RegionalRoute {
                    match self {
                        #(
                            Self::#route_idents => RegionalRoute::#lor_idents,
                        )*
                    }
                }

                /// Used in the LoL Tournament API. Specifically
                /// [`tournament-stub-v5.registerProviderData`](crate::endpoints::TournamentStubV5::register_provider_data)
                /// and [`tournament-v5.registerProviderData`](crate::endpoints::TournamentV5::register_provider_data).
                ///
                /// Returns `None` if the corresponding tournament region is unknown: <https://github.com/MingweiSamuel/riotapi-schema/issues/58>.
                pub fn to_tournament_region(self) -> Option<TournamentRegion> {
                    match self {
                        #(
                            Self::#route_idents => #tournament_regions,
                        )*
                    }
                }

                /// Get the slightly more human-friendly alternate name for this `PlatformRoute`. Specifically
                /// excludes any trailing numbers and appends extra N(orth), S(outh), E(ast), and/or W(est)
                /// suffixes to some names. Some of these are old region names which are often still used as
                /// user-facing names, e.g. on op.gg.
                ///
                /// Note these strings *are* handled by the `FromStr` implementation, if you wish to parse them
                /// back into `PlatformRoute`s.
                pub fn as_region_str(self) -> &'static str {
                    match self {
                        #(
                            Self::#route_idents => #alt_names,
                        )*
                    }
                }
            }
        }
    };

    let val_platform_route = make_enum(
        &routes_table.val_platform,
        parse_quote!(ValPlatformRoute),
        "Platform routes for Valorant.",
    );

    let tournament_region = {
        let mut variants = routes_table
            .platform
            .values()
            .filter_map(|route| {
                route
                    .tournament_region
                    .as_deref()
                    .map(|tournament_region| (tournament_region, route))
            })
            .collect::<Vec<_>>();
        variants.sort_unstable_by_key(|(_, region)| region.id);

        let variants = variants.into_iter().map(
            |(
                tournament_region,
                Route {
                    id,
                    description,
                    deprecated,
                    ..
                },
            )| {
                let docs = description.lines().map(str::trim);
                let doc_repr = format!("`{}` (riotapi-schema ID/repr)", id);
                let deprecated_attr = deprecated
                    .unwrap_or_default()
                    .then(|| quote!(#[deprecated]));
                let key = Ident::new(tournament_region, Span::call_site());
                let value = Literal::u8_unsuffixed(*id);

                quote! {
                    #( #[doc = #docs] )*
                    ///
                    #[doc = #doc_repr]
                    #deprecated_attr
                    #key = #value,
                }
            },
        );
        quote! {
            /// Tournament regions for League of Legends (LoL) used in
            /// [`TournamentStubV5`](crate::endpoints::TournamentStubV5)
            /// and [`TournamentV5`](crate::endpoints::TournamentV5).
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
            #[derive(IntoPrimitive, TryFromPrimitive)]
            #[derive(EnumString, EnumIter, Display, IntoStaticStr)]
            #[derive(serde::Serialize, crate::de::Deserialize)]
            #[repr(u8)]
            #[non_exhaustive]
            pub enum TournamentRegion {
                #(
                    #variants
                )*
            }
        }
    };

    parse_quote! {
        use num_enum::{IntoPrimitive, TryFromPrimitive};
        use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

        #regional_route

        #platform_route
        #platform_route_impl

        #val_platform_route

        #tournament_region
    }
}
