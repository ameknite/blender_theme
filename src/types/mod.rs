// SPDX-License-Identifier: GPL-3.0-or-later

use core::fmt;

use thiserror::Error;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq)]
pub struct Factor(f32);

impl Factor {
    pub fn new(value: f32) -> Result<Self, FactorError> {
        if (0.0..=1.0).contains(&value) {
            Ok(Factor(value))
        } else {
            Err(FactorError::OutOfRange(value))
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
#[derive(Debug, Error)]
pub enum FactorError {
    #[error("Factor value must be between 0.0 and 1.0, but got: {0}")]
    OutOfRange(f32),
}

impl<'de> Deserialize<'de> for Factor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        Factor::new(value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Factor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct HexColor {
    value: u32,
    has_alpha: bool,
}

impl HexColor {
    pub fn new(value: u32, has_alpha: bool) -> Self {
        Self { value, has_alpha }
    }

    pub fn new_from_hex(hex_str: &str) -> Result<Self, HexColorError> {
        if !hex_str.starts_with('#') {
            return Err(HexColorError::MissingHash(hex_str.to_string()));
        }

        let without_hash = &hex_str[1..];
        let value = match without_hash.len() {
            6 => {
                // Handle #RRGGBB format
                Self::new(u32::from_str_radix(without_hash, 16)?, false)
            }
            8 => {
                // Handle #RRGGBBAA format directly
                Self::new(u32::from_str_radix(without_hash, 16)?, true)
            }
            _ => {
                return Err(HexColorError::IncorrectLength(hex_str.to_string()))
            }
        };

        Ok(value)
    }

    pub fn to_hex_string(&self) -> String {
        if self.has_alpha {
            format!("#{:08x}", self.value)
        } else {
            format!("#{:06x}", self.value)
        }
    }
}

#[derive(Debug, Error)]
pub enum HexColorError {
    #[error("Missing the leading '#': {0}")]
    MissingHash(String),
    #[error("Color string has incorrect length (should be 7 or 9, counting the hash #): {0}")]
    IncorrectLength(String),
    #[error("Failed to parse hex value: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        HexColor::new_from_hex(&s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let color_string = self.to_hex_string();
        serializer.serialize_str(&color_string)
    }
}

#[derive(Debug, PartialEq)]
pub struct Boolean(bool);

impl Boolean {
    pub fn new_from_str(value_str: &str) -> Result<Self, BooleanError> {
        match value_str {
            "TRUE" => Ok(Boolean(true)),
            "FALSE" => Ok(Boolean(false)),
            _ => Err(BooleanError::UnexpectedValue(value_str.to_string())),
        }
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.0 { "TRUE" } else { "FALSE" })
    }
}

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("Unexpected value: '{0}'. Expected 'TRUE' or 'FALSE'.")]
    UnexpectedValue(String),
}

impl Serialize for Boolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let boolean_str = self.to_string();
        serializer.serialize_str(&boolean_str)
    }
}

impl<'de> Deserialize<'de> for Boolean {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Boolean::new_from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BackGroundType {
    RADIAL,
    #[serde(rename = "SINGLE_COLOR")]
    SingleColor,
}
