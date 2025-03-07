use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, File, Ident};

use super::enum_unknown::EnumString;

pub fn game_type(variants: &[EnumString]) -> File {
    let variants = variants.iter().map(
        |EnumString {
             x_name,
             x_desc,
             x_deprecated,
             notes,
         }| {
            assert_eq!(
                None, *x_deprecated,
                "Unexpectedly deprecated `GameType` {}.",
                x_name
            );
            assert_eq!(None, *notes, "Unexpected notes for `GameType` {}.", x_name);

            let docs = x_desc.lines().map(str::trim);
            let name_no_game = x_name.strip_suffix("_GAME").unwrap_or(x_name);
            let key = Ident::new(x_name, Span::call_site());
            quote! {
                #( #[doc = #docs] )*
                // Note: strum(serialize = ...) actually specifies extra **De**serialization values.
                #[strum(to_string = #x_name, serialize = #name_no_game)]
                #[serde(alias = #name_no_game)]
                #key,
            }
        },
    );

    parse_quote! {
        use strum_macros::{ EnumString, Display, AsRefStr, IntoStaticStr };

        /// League of Legends game type: matched game, custom game, or tutorial game.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[derive(EnumString, Display, AsRefStr, IntoStaticStr)]
        #[derive(serde::Serialize, crate::de::Deserialize)]
        #[repr(u8)]
        pub enum GameType {
            #( #variants )*
        }
    }
}
