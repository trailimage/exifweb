use crate::num_traits::FromPrimitive;
use fmt::Display;
use serde::{de, Deserialize, Deserializer};
use std::{fmt, marker::Copy};

#[derive(Debug, Primitive, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExposureMode {
    Undefined = 0,
    Manual = 1,
    ProgramAE = 2,
    AperturePriority = 3,
    ShutterPriority = 4,
    Creative = 5,
    Action = 6,
    Portrait = 7,
    Landscape = 8,
    Bulb = 9,
}

impl Clone for ExposureMode {
    fn clone(&self) -> ExposureMode {
        *self
    }
}

impl Default for ExposureMode {
    fn default() -> Self {
        ExposureMode::Undefined
    }
}

impl<'de> Deserialize<'de> for ExposureMode {
    fn deserialize<D>(deserializer: D) -> Result<ExposureMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(ExposureModeVisitor)
    }
}

impl Display for ExposureMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExposureMode::Undefined => f.write_str("Unknown"),
            ExposureMode::Manual => f.write_str("Manual"),
            ExposureMode::ProgramAE => f.write_str("Program AE"),
            ExposureMode::AperturePriority => f.write_str("Aperture Priority"),
            ExposureMode::ShutterPriority => f.write_str("Shutter Priority"),
            ExposureMode::Creative => f.write_str("Creative Mode"),
            ExposureMode::Action => f.write_str("Action Mode"),
            ExposureMode::Portrait => f.write_str("Portrait Mode"),
            ExposureMode::Landscape => f.write_str("Landscape Mode"),
            ExposureMode::Bulb => f.write_str("Bulb Flash"),
        }
    }
}

struct ExposureModeVisitor;

impl<'de> de::Visitor<'de> for ExposureModeVisitor {
    type Value = ExposureMode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 9")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ExposureMode::from_u64(value).unwrap())
    }
}
