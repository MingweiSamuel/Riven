#![macro_use]

macro_rules! serde_string {
    ( $name:ident ) => {
        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>
            {
                let s = String::deserialize(deserializer)?;
                s.parse().map_err(serde::de::Error::custom)
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
    };
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

macro_rules! newtype_enum {
    {
        $( #[$attr:meta] )*
        $v:vis newtype_enum $name:ident($repr:ident) {
            $(
                $( #[$var_attr:meta] )*
                $var_name:ident = $var_val:expr,
            )*
        }
    } => {
        $( #[$attr] )*
        #[derive(Copy, Clone)]
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        $v struct $name($v $repr);
        impl $name {
            $(
                $( #[$var_attr] )*
                $v const $var_name: Self = Self( $var_val );
            )*
        }

        impl $name {
            arr!{
                #[doc = "Array containing all variants, ordered by their id value."]
                pub const ALL_KNOWN: [Self; _] = [
                    $( Self::$var_name, )*
                ]
            }

            #[doc = "If this is one of the known variants."]
            $v fn is_known(self) -> bool {
                match self {
                    $(
                        Self::$var_name => true,
                    )*
                    _ => false,
                }
            }
        }

        impl std::convert::From<$repr> for $name {
            fn from(value: $repr ) -> Self {
                Self(value)
            }
        }

        impl std::convert::From<$name> for $repr {
            fn from(value: $name ) -> Self {
                value.0
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
