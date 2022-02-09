#![macro_use]

macro_rules! string_enum_str {
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