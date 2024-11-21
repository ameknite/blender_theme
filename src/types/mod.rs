// SPDX-License-Identifier: GPL-3.0-or-later

use core::fmt;

use thiserror::Error;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq)]
pub struct Factor(f32);

impl Factor {
    pub fn try_from_f32(value: f32) -> Result<Self, FactorError> {
        if (0.0..=1.0).contains(&value) {
            Ok(Factor(value))
        } else {
            Err(FactorError::OutOfRange(value))
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn to_hex_str(&self) -> String {
        format!("{:02x}", (self.value() * 255.0) as u8)
    }
}

impl TryFrom<f32> for Factor {
    type Error = FactorError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Factor::try_from_f32(value)
    }
}

#[derive(Clone, Debug, Error)]
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
        Factor::try_from_f32(value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Factor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.value())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HexColor {
    pub value: Rgb,
    pub alpha: Option<u8>,
}

impl HexColor {
    fn new(value: Rgb, alpha: Option<u8>) -> Self {
        Self { value, alpha }
    }

    pub fn try_from_str(hex_str: &str) -> Result<Self, HexColorError> {
        if !hex_str.starts_with('#') {
            return Err(HexColorError::MissingHash(hex_str.to_string()));
        }

        let without_hash = &hex_str[1..];
        let value = match without_hash.len() {
            6 => {
                // Handle #RRGGBB format
                let array: [u8; 3] = const_hex::decode_to_array(without_hash)?;
                let r = array[0];
                let g = array[1];
                let b = array[2];
                Self::new(Rgb { r, g, b }, None)
            }
            8 => {
                // Handle #RRGGBBAA format
                let without_hash: [u8; 4] =
                    const_hex::decode_to_array(without_hash)?;
                let r = without_hash[0];
                let g = without_hash[1];
                let b = without_hash[2];
                let a = without_hash[3];
                Self::new(Rgb { r, g, b }, Some(a))
            }
            _ => {
                return Err(HexColorError::IncorrectLength(hex_str.to_string()))
            }
        };

        Ok(value)
    }

    pub fn to_hex_string(&self) -> String {
        let rgb = const_hex::encode([self.value.r, self.value.g, self.value.b]);
        let mut hex_string = "#".to_string() + &rgb;

        if let Some(alpha) = self.alpha {
            hex_string += &const_hex::encode([alpha]);
        }

        hex_string
    }
}

impl TryFrom<&str> for HexColor {
    type Error = HexColorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        HexColor::try_from_str(value)
    }
}

impl TryFrom<String> for HexColor {
    type Error = HexColorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        HexColor::try_from_str(&value)
    }
}

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

#[derive(Clone, Debug, Error)]
pub enum HexColorError {
    #[error("Missing the leading hash '#': {0}")]
    MissingHash(String),
    #[error("Color string has incorrect length (should be 7 or 9, counting the hash #): {0}")]
    IncorrectLength(String),
    #[error("Error decoding hex string to Array: {0}")]
    HexError(#[from] const_hex::FromHexError),
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        HexColor::try_from_str(&s).map_err(serde::de::Error::custom)
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, Error)]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum BackGroundType {
    RADIAL,
    #[serde(rename = "SINGLE_COLOR")]
    SingleColor,
}
