// use serde::{ Serialize, Deserialize };
// use strum_macros::{ EnumString, Display, AsRefStr, IntoStaticStr };

// /// LoL or TFT ranked queue types.
// #[non_exhaustive]
// #[derive(Debug, Clone)]
// #[derive(Eq, PartialEq, Hash)]
// #[derive(EnumString, Display, AsRefStr, IntoStaticStr)]
// #[derive(Deserialize)]
// #[serde(try_from = "&str")]
// pub enum QueueType {
//     /// Catch-all variant for new, unknown queue types.
//     #[strum(default)]
//     UNKNOWN(String),

//     /// League of Legends, Summoner's Rift (5v5), Ranked Solo Queue.
//     RANKED_SOLO_5x5,
//     /// League of Legends, Summoner's Rift (5v5), Flex Queue.
//     RANKED_FLEX_SR,
//     /// League of Legends, Twisted Treeline (3v3), Flex Queue.
//     RANKED_FLEX_TT,
//     /// Ranked Teamfight Tactics.
//     RANKED_TFT,
//     /// Ranked Teamfight Tactics, Hyper Roll gamemode.
//     RANKED_TFT_TURBO,
//     /// Ranked Teamfight Tactics, Double Up gamemode.
//     RANKED_TFT_DOUBLE_UP,

//     /// Ranked Teamfight Tactics, OLD Double Up gamemode. Changed some time before June 2022.
//     #[deprecated(note="Use RANKED_TFT_DOUBLE_UP instead.")]
//     RANKED_TFT_PAIRS,
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn check_as_ref() {
//         assert_eq!("RANKED_SOLO_5x5", QueueType::RANKED_SOLO_5x5.as_ref());
//     }

//     #[test]
//     fn check_to_string() {
//         assert_eq!("RANKED_SOLO_5x5", QueueType::RANKED_SOLO_5x5.to_string());
//     }

//     #[test]
//     fn check_from_string() {
//         assert_eq!(Some(QueueType::RANKED_SOLO_5x5), "RANKED_SOLO_5x5".parse().ok());
//     }

//     #[test]
//     fn check_deserialize_unknown() {
//         use std::collections::HashMap;

//         let s = r#"{
//             "a": "RANKED_SOLO_5x5",
//             "b": "RANKED_TFT_DOUBLE_UP",
//             "c": "RANKED_TFT_PAIRS",
//             "d": "RANKED_UNKNOWN_ASDF"
//         }
//         "#;
//         let dict: HashMap<String, QueueType> = serde_json::from_str(s).expect("Failed to parse.");
//         let expected: HashMap<String, QueueType> = std::iter::IntoIterator::into_iter([
//             ("a", QueueType::RANKED_SOLO_5x5),
//             ("b", QueueType::RANKED_TFT_DOUBLE_UP),
//             ("c", QueueType::RANKED_TFT_PAIRS),
//             ("d", QueueType::UNKNOWN("RANKED_UNKNOWN_ASDF".to_owned())),
//         ]).map(|(k, v)| (k.to_owned(), v)).collect();
//         assert_eq!(expected, dict);
//     }
// }


use serde::{Serialize, Deserialize};
    use strum_macros::{EnumString, Display, AsRefStr, IntoStaticStr};
    /// LoL or TFT ranked queue types.
    #[non_exhaustive]
    // #[serde(try_from = "&str")]
    pub enum QueueType {
        /// Catch-all variant for new, unknown queue types.
        // #[strum(default)]
        UNKNOWN(String),
        /// League of Legends, Summoner's Rift (5v5), Ranked Solo Queue.
        RANKED_SOLO_5x5,
        /// League of Legends, Summoner's Rift (5v5), Flex Queue.
        RANKED_FLEX_SR,
        /// League of Legends, Twisted Treeline (3v3), Flex Queue.
        RANKED_FLEX_TT,
        /// Ranked Teamfight Tactics.
        RANKED_TFT,
        /// Ranked Teamfight Tactics, Hyper Roll gamemode.
        RANKED_TFT_TURBO,
        /// Ranked Teamfight Tactics, Double Up gamemode.
        RANKED_TFT_DOUBLE_UP,
        /// Ranked Teamfight Tactics, OLD Double Up gamemode. Changed some time before June 2022.
        #[deprecated(note = "Use RANKED_TFT_DOUBLE_UP instead.")]
        RANKED_TFT_PAIRS,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for QueueType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                _serde::__private::Result::and_then(
                    <&str as _serde::Deserialize>::deserialize(__deserializer),
                    |v| _serde::__private::TryFrom::try_from(v).map_err(_serde::de::Error::custom),
                )
            }
        }
    };
    impl ::std::str::FromStr for QueueType {
        type Err = ::strum::ParseError;
        fn from_str(s: &str) -> ::std::result::Result<QueueType, Self::Err> {
            match s {
                "RANKED_SOLO_5x5" => ::std::result::Result::Ok(QueueType::RANKED_SOLO_5x5),
                "RANKED_FLEX_SR" => ::std::result::Result::Ok(QueueType::RANKED_FLEX_SR),
                "RANKED_FLEX_TT" => ::std::result::Result::Ok(QueueType::RANKED_FLEX_TT),
                "RANKED_TFT" => ::std::result::Result::Ok(QueueType::RANKED_TFT),
                "RANKED_TFT_TURBO" => ::std::result::Result::Ok(QueueType::RANKED_TFT_TURBO),
                "RANKED_TFT_DOUBLE_UP" => {
                    ::std::result::Result::Ok(QueueType::RANKED_TFT_DOUBLE_UP)
                }
                "RANKED_TFT_PAIRS" => ::std::result::Result::Ok(QueueType::RANKED_TFT_PAIRS),
                default => ::std::result::Result::Ok(QueueType::UNKNOWN(default.into())),
            }
        }
    }
    impl ::std::fmt::Display for QueueType {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            match *self {
                QueueType::UNKNOWN(..) => f.pad("UNKNOWN"),
                QueueType::RANKED_SOLO_5x5 => f.pad("RANKED_SOLO_5x5"),
                QueueType::RANKED_FLEX_SR => f.pad("RANKED_FLEX_SR"),
                QueueType::RANKED_FLEX_TT => f.pad("RANKED_FLEX_TT"),
                QueueType::RANKED_TFT => f.pad("RANKED_TFT"),
                QueueType::RANKED_TFT_TURBO => f.pad("RANKED_TFT_TURBO"),
                QueueType::RANKED_TFT_DOUBLE_UP => f.pad("RANKED_TFT_DOUBLE_UP"),
                QueueType::RANKED_TFT_PAIRS => f.pad("RANKED_TFT_PAIRS"),
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueueType {
        fn as_ref(&self) -> &str {
            match *self {
                QueueType::UNKNOWN(..) => "UNKNOWN",
                QueueType::RANKED_SOLO_5x5 => "RANKED_SOLO_5x5",
                QueueType::RANKED_FLEX_SR => "RANKED_FLEX_SR",
                QueueType::RANKED_FLEX_TT => "RANKED_FLEX_TT",
                QueueType::RANKED_TFT => "RANKED_TFT",
                QueueType::RANKED_TFT_TURBO => "RANKED_TFT_TURBO",
                QueueType::RANKED_TFT_DOUBLE_UP => "RANKED_TFT_DOUBLE_UP",
                QueueType::RANKED_TFT_PAIRS => "RANKED_TFT_PAIRS",
            }
        }
    }
    impl ::std::convert::From<QueueType> for &'static str {
        fn from(x: QueueType) -> &'static str {
            match x {
                QueueType::UNKNOWN(..) => "UNKNOWN",
                QueueType::RANKED_SOLO_5x5 => "RANKED_SOLO_5x5",
                QueueType::RANKED_FLEX_SR => "RANKED_FLEX_SR",
                QueueType::RANKED_FLEX_TT => "RANKED_FLEX_TT",
                QueueType::RANKED_TFT => "RANKED_TFT",
                QueueType::RANKED_TFT_TURBO => "RANKED_TFT_TURBO",
                QueueType::RANKED_TFT_DOUBLE_UP => "RANKED_TFT_DOUBLE_UP",
                QueueType::RANKED_TFT_PAIRS => "RANKED_TFT_PAIRS",
            }
        }
    }
    impl<'_derivative_strum> ::std::convert::From<&'_derivative_strum QueueType> for &'static str {
        fn from(x: &'_derivative_strum QueueType) -> &'static str {
            match *x {
                QueueType::UNKNOWN(..) => "UNKNOWN",
                QueueType::RANKED_SOLO_5x5 => "RANKED_SOLO_5x5",
                QueueType::RANKED_FLEX_SR => "RANKED_FLEX_SR",
                QueueType::RANKED_FLEX_TT => "RANKED_FLEX_TT",
                QueueType::RANKED_TFT => "RANKED_TFT",
                QueueType::RANKED_TFT_TURBO => "RANKED_TFT_TURBO",
                QueueType::RANKED_TFT_DOUBLE_UP => "RANKED_TFT_DOUBLE_UP",
                QueueType::RANKED_TFT_PAIRS => "RANKED_TFT_PAIRS",
            }
        }
    }
    impl ::core::marker::StructuralEq for QueueType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for QueueType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for QueueType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for QueueType {
        #[inline]
        fn eq(&self, other: &QueueType) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&QueueType::UNKNOWN(ref __self_0), &QueueType::UNKNOWN(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &QueueType) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&QueueType::UNKNOWN(ref __self_0), &QueueType::UNKNOWN(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => false,
                    }
                } else {
                    true
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for QueueType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                (&QueueType::UNKNOWN(ref __self_0),) => {
                    ::core::hash::Hash::hash(&::core::intrinsics::discriminant_value(self), state);
                    ::core::hash::Hash::hash(&(*__self_0), state)
                }
                _ => ::core::hash::Hash::hash(&::core::intrinsics::discriminant_value(self), state),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for QueueType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&QueueType::UNKNOWN(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "UNKNOWN");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&QueueType::RANKED_SOLO_5x5,) => {
                    ::core::fmt::Formatter::write_str(f, "RANKED_SOLO_5x5")
                }
                (&QueueType::RANKED_FLEX_SR,) => {
                    ::core::fmt::Formatter::write_str(f, "RANKED_FLEX_SR")
                }
                (&QueueType::RANKED_FLEX_TT,) => {
                    ::core::fmt::Formatter::write_str(f, "RANKED_FLEX_TT")
                }
                (&QueueType::RANKED_TFT,) => ::core::fmt::Formatter::write_str(f, "RANKED_TFT"),
                (&QueueType::RANKED_TFT_TURBO,) => {
                    ::core::fmt::Formatter::write_str(f, "RANKED_TFT_TURBO")
                }
                (&QueueType::RANKED_TFT_DOUBLE_UP,) => {
                    ::core::fmt::Formatter::write_str(f, "RANKED_TFT_DOUBLE_UP")
                }
                (&QueueType::RANKED_TFT_PAIRS,) => {
                    ::core::fmt::Formatter::write_str(f, "RANKED_TFT_PAIRS")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for QueueType {
        #[inline]
        fn clone(&self) -> QueueType {
            match (&*self,) {
                (&QueueType::UNKNOWN(ref __self_0),) => {
                    QueueType::UNKNOWN(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&QueueType::RANKED_SOLO_5x5,) => QueueType::RANKED_SOLO_5x5,
                (&QueueType::RANKED_FLEX_SR,) => QueueType::RANKED_FLEX_SR,
                (&QueueType::RANKED_FLEX_TT,) => QueueType::RANKED_FLEX_TT,
                (&QueueType::RANKED_TFT,) => QueueType::RANKED_TFT,
                (&QueueType::RANKED_TFT_TURBO,) => QueueType::RANKED_TFT_TURBO,
                (&QueueType::RANKED_TFT_DOUBLE_UP,) => QueueType::RANKED_TFT_DOUBLE_UP,
                (&QueueType::RANKED_TFT_PAIRS,) => QueueType::RANKED_TFT_PAIRS,
            }
        }
    }