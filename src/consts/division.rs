#![allow(deprecated)]

use strum_macros::{ EnumString, Display, AsRefStr };
use serde_repr::{ Serialize_repr, Deserialize_repr };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(EnumString, Display, AsRefStr)]
#[derive(Serialize_repr, Deserialize_repr)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Division {
    I   = 1,
    II  = 2,
    III = 3,
    IV  = 4,
    #[deprecated(note="Removed for 2019.")]
    V   = 5,
}
