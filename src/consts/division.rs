#![allow(deprecated)]

use std::fmt::Debug;

use strum_macros::{ EnumString, Display, AsRefStr };

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(EnumString, Display, AsRefStr)]
#[repr(u8)]
pub enum Division {
    I   = 1,
    II  = 2,
    III = 3,
    IV  = 4,

    #[deprecated(note="Removed for 2019.")]
    V   = 5,
}
