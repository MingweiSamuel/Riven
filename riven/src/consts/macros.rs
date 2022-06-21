#![macro_use]

/// Macro for deriving `Serialize` and `Deserialize` for string enums with an
/// `UNKNOWN(String)` variant.
///
/// Enum should have `#[derive(EnumString, EnumVariantNames, IntoStaticStr)]` included.
///
/// Also implements `AsRef<str>`, `Display`, and `From<&str>`.
macro_rules! serde_strum_unknown {
    ( $name:ident ) => {
        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                match self {
                    Self::UNKNOWN(string) => &*string,
                    known => known.into(),
                }
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                self.as_ref().fmt(f)
            }
        }
        impl serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_ref())
            }
        }

        impl From<&str> for $name {
            fn from(item: &str) -> Self {
                item.parse().unwrap()
            }
        }
        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>
            {
                #[cfg(not(feature = "deny-unknown-enum-variants-strings"))]
                {
                    <&str>::deserialize(deserializer).map(Into::into)
                }
                #[cfg(feature = "deny-unknown-enum-variants-strings")]
                {
                    <&str>::deserialize(deserializer).map(Into::into)
                        .and_then(|item| {
                            match item {
                                Self::UNKNOWN(unknown) => Err(serde::de::Error::unknown_variant(
                                    &*unknown,
                                    <Self as strum::VariantNames>::VARIANTS,
                                )),
                                other => Ok(other),
                            }
                        })
                }
            }
        }
    }
}

macro_rules! arr {
    (
        $( #[$attr:meta] )*
        $v:vis $id:ident $name:ident: [$ty:ty; _] = $value:expr
    ) => {
        $( #[$attr] )*
        $v $id $name: [$ty; $value.len()] = $value;
    }
}

/// Macro for newtype "enums" with integer values.
///
/// For serde, use the following:
/// ```ignore
/// #[derive(Serialize, Deserialize)]
/// #[serde(from = "$repr", into = "$repr")]
/// ```
macro_rules! newtype_enum {
    {
        $( #[$attr:meta] )*
        $v:vis newtype_enum $name:ident($repr:ty) {
            $(
                $( #[$var_attr:meta] )*
                $var_name:ident = $var_val:expr,
            )*
        }
    } => {
        $( #[$attr] )*
        #[derive(Copy, Clone)]
        #[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        $v struct $name($v $repr);
        impl $name {
            $(
                $( #[$var_attr] )*
                $v const $var_name: Self = Self($var_val);
            )*
        }

        impl $name {
            arr!{
                #[doc = "Array containing all known variants."]
                pub const ALL_KNOWN: [Self; _] = [
                    $( Self::$var_name, )*
                ]
            }

            #[doc = "If this is one of the known variants."]
            $v const fn is_known(self) -> bool {
                match self {
                    $(
                        Self::$var_name => true,
                    )*
                    _ => false,
                }
            }
        }

        impl std::convert::From<$name> for $repr {
            fn from(value: $name ) -> Self {
                value.0
            }
        }
        impl serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                <$repr>::serialize(&self.0, serializer)
            }
        }

        impl std::convert::From<$repr> for $name {
            fn from(value: $repr ) -> Self {
                Self(value)
            }
        }
        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>
            {
                #[cfg(not(feature = "deny-unknown-enum-variants-integers"))]
                {
                    <$repr>::deserialize(deserializer).map(Into::into)
                }
                #[cfg(feature = "deny-unknown-enum-variants-integers")]
                {
                    <$repr>::deserialize(deserializer).map(Into::into)
                        .and_then(|item: Self| {
                            if !item.is_known() {
                                Err(serde::de::Error::custom(format!(
                                    "Unknown integer enum variant: {} (\"deny-unknown-enum-variants-integers\" feature is enabled).\nExpected one of the following: {:?}",
                                    item, Self::ALL_KNOWN
                                )))
                            }
                            else {
                                Ok(item)
                            }
                        })
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({}{})", stringify!($name), self.0, if self.is_known() { "" } else { "?" })
            }
        }
    }
}
