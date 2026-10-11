//! Typed view of `config/balance.toml`, embedded at build time.

use serde::Deserialize;
use thiserror::Error;

use crate::civ::CivId;

/// Raw text of `config/balance.toml`, embedded so every build of the sim
/// carries exactly one balance.
pub const BALANCE_TOML: &str = include_str!("../../../config/balance.toml");

#[derive(Debug, Error)]
pub enum BalanceError {
    #[error("balance.toml is not valid: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("balance.toml: {0}")]
    Invalid(&'static str),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub time: Time,
    pub world: WorldShape,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Time {
    pub ticks_per_second: u32,
    pub ticks_per_day: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldShape {
    pub islands: u32,
    pub placeholder_walkers_per_island: u32,
    pub island_radius: f32,
    pub walker_speed: f32,
}

impl Balance {
    /// Parses and validates the embedded `config/balance.toml`.
    ///
    /// # Errors
    /// Returns an error if the embedded TOML does not match [`Balance`] or
    /// violates an invariant.
    pub fn embedded() -> Result<Self, BalanceError> {
        Self::parse(BALANCE_TOML)
    }

    /// Parses and validates a balance document.
    ///
    /// # Errors
    /// Returns an error if `text` does not match [`Balance`] or violates an
    /// invariant.
    pub fn parse(text: &str) -> Result<Self, BalanceError> {
        let balance: Balance = toml::from_str(text)?;
        balance.validate()?;
        Ok(balance)
    }

    fn validate(&self) -> Result<(), BalanceError> {
        if self.time.ticks_per_second == 0 || self.time.ticks_per_day == 0 {
            return Err(BalanceError::Invalid("time values must be positive"));
        }
        if self.world.islands as usize != CivId::ALL.len() {
            return Err(BalanceError::Invalid(
                "world.islands must equal the number of civs",
            ));
        }
        if !(self.world.island_radius > 0.0 && self.world.walker_speed > 0.0) {
            return Err(BalanceError::Invalid(
                "world radius and speed must be positive",
            ));
        }
        Ok(())
    }
}

/// Hash of the embedded balance text; stored in snapshots so a runner refuses
/// to resume a season with different tuning.
pub fn balance_hash() -> u64 {
    xxhash_rust::xxh3::xxh3_64(BALANCE_TOML.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::{Balance, BalanceError};

    #[test]
    fn embedded_balance_is_valid() {
        let balance = Balance::embedded().expect("embedded balance parses");
        assert_eq!(balance.world.islands, 5);
    }

    #[test]
    fn unknown_keys_are_rejected() {
        let text = format!("{}\n[extra]\nx = 1\n", super::BALANCE_TOML);
        assert!(matches!(Balance::parse(&text), Err(BalanceError::Parse(_))));
    }
}
