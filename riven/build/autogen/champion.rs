use std::collections::BTreeMap;

use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use syn::{parse_quote, File};

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Champion {
    pub id: i16,
    pub name: String,
    pub alias: String,
    pub square_portrait_path: Option<String>,
    pub roles: Vec<String>,
}
impl Champion {
    fn ident(&self) -> Ident {
        Ident::new(&self.alias.to_shouty_snake_case(), Span::call_site())
    }
}

pub fn champion(champions: &[Champion]) -> File {
    let champions = champions.iter().filter(|&champion| 0 <= champion.id);

    let newtype_enum = {
        let doc_rows = champions.clone().map(|champion| {
            let id = champion.id;
            let name = &champion.name;
            let alias = &champion.alias;
            format!("`{}` | {} | {} | {}", id, name, alias, id)
        });

        let variants = champions.clone().map(|champion| {
            let doc = format!("`{}`", champion.id);
            let ident = champion.ident();
            let id = Literal::i16_unsuffixed(champion.id);
            quote! {
                #[doc = #doc]
                #ident = #id,
            }
        });

        quote! {
            #[macro_rules_attribute::apply(newtype_enum)]
            #[repr(i16)]
            /// A League of Legends champion.
            ///
            /// This newtype acts as a C-like enum; each variant corresponds to an
            /// integer value. Using a newtype allows _unknown_ variants to be
            /// represented. This is important when Riot adds new champions.
            ///
            /// Field | Name | Identifier | Id
            /// ---|---|---|---
            /// `NONE` | None (no ban) | | -1
            #( #[doc = #doc_rows ] )*
            pub enum Champion {
                /// `-1`, none. Appears when a champion ban is not used in champ select.
                NONE = -1,
                #( #variants )*
            }
        }
    };

    let champion_impl = {
        let name_cases = champions.clone().map(|champion| {
            let ident = champion.ident();
            let name = &champion.name;
            quote! {
                Self::#ident => Some(#name),
            }
        });
        let identifier_doc_rows = champions
            .clone()
            .filter(|champion| {
                champion.alias
                    != champion
                        .name
                        .replace(|c| char::is_ascii_alphanumeric(&c), "")
            })
            .map(|champion| {
                let id = champion.id;
                let name = &champion.name;
                let alias = &champion.alias;
                format!("`{}` | {} | {} | {}", id, name, alias, id)
            });
        let identifier_cases = champions.clone().map(|champion| {
            let ident = champion.ident();
            let alias = &champion.alias;
            quote! {
                Self::#ident => Some(#alias),
            }
        });

        quote! {
            impl Champion {
                /// The champion's name (`en_US` localization).
                pub const fn name(self) -> Option<&'static str> {
                    match self {
                        #( #name_cases )*
                        _ => None,
                    }
                }

                /// The champion's identifier key. Somtimes called "key", "identifier", or "alias".
                /// This is mainly used in DDragon paths.
                ///
                /// This is generally the `en_US` name with spaces and punctuation removed,
                /// capitalization preserved, however the follow are exceptions:
                ///
                /// Field | Name | Identifier | Id
                /// ---|---|---|---
                #( #[doc = #identifier_doc_rows] )*
                pub const fn identifier(self) -> Option<&'static str> {
                    match self {
                        #( #identifier_cases )*
                        _ => None,
                    }
                }
            }
        }
    };

    let champion_parse = {
        let cases: BTreeMap<String, &Champion> = champions
            .flat_map(|champion| {
                IntoIterator::into_iter([&champion.alias, &champion.name]).flat_map(move |s| {
                    // e.g. `CHOG` for `Cho'Gath`.
                    let chars_full = s.chars().filter(|c| c.is_ascii_alphanumeric());
                    // e.g. `CHO` for `Cho'Gath`.
                    let chars_first = s.chars().take_while(|c| c.is_ascii_alphanumeric());

                    IntoIterator::into_iter([
                        Box::new(chars_full) as Box<dyn Iterator<Item = char>>,
                        Box::new(chars_first),
                    ])
                    .map(move |chars| {
                        let str: String = chars
                            .map(|c| c.to_ascii_uppercase())
                            .chain(std::iter::repeat('\0'))
                            .take(4)
                            .collect();
                        (str, champion)
                    })
                })
            })
            .collect();
        let cases = cases.into_iter().map(|(s, champion)| {
            let ident = champion.ident();
            let chars = s.chars().map(Literal::character);
            quote! {
                [#( #chars ),*] => Ok(Champion::#ident),
            }
        });

        quote! {
            impl std::str::FromStr for Champion {
                type Err = ParseChampionError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let mut chars = ['\0'; 4];
                    s.chars()
                        .take(4)
                        .filter(|c| c.is_ascii_alphanumeric())
                        .map(|c| c.to_ascii_uppercase())
                        .enumerate()
                        .for_each(|(i, c)| chars[i] = c);
                    match chars {
                        #( #cases )*
                        unknown => Err(ParseChampionError(unknown)),
                    }
                }
            }
        }
    };

    parse_quote! {
        #newtype_enum
        #champion_impl
        #champion_parse
    }
}
