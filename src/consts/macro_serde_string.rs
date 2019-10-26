#![macro_use]
#[macro_export]
macro_rules! serde_string {
    ( $x:ty ) => {
        impl<'de> serde::de::Deserialize<'de> for $x
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>
            {
                let s = String::deserialize(deserializer)?;
                s.parse().map_err(serde::de::Error::custom)
            }
        }
        impl serde::ser::Serialize for $x {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_ref())
            }
        }
    };
}
