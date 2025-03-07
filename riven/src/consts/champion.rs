include_autogen!("champion.gen.rs");

impl Champion {
    /// https://github.com/MingweiSamuel/Riven/issues/36
    pub(crate) fn serialize_result<S>(
        val: &Result<Self, std::num::TryFromIntError>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::Serialize;
        val.unwrap_or(Champion(-1)).serialize(serializer)
    }

    /// https://github.com/MingweiSamuel/Riven/issues/36
    pub(crate) fn deserialize_result<'de, D>(
        deserializer: D,
    ) -> Result<Result<Self, std::num::TryFromIntError>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use std::convert::TryInto;
        <i64 as serde::de::Deserialize>::deserialize(deserializer).map(|id| id.try_into().map(Self))
    }
}

/// The error used for failures in [`Champion`]'s
/// [`FromStr`](std::str::FromStr) implementation.
///
/// Currently only internally stores the four characters used to parse the
/// champion, but may change in the future.
#[derive(Debug)]
pub struct ParseChampionError([char; 4]);
impl std::fmt::Display for ParseChampionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: String = self.0.iter().copied().take_while(|&c| '\0' != c).collect();
        write!(f, "Failed to parse unknown champion prefix: {:?}", s)
    }
}
impl std::error::Error for ParseChampionError {}

impl std::convert::TryFrom<&str> for Champion {
    type Error = <Self as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        <Self as std::str::FromStr>::from_str(value)
    }
}
