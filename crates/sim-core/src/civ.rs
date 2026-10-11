//! Civilization ids. Index order is part of the determinism contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum CivId {
    Vesper = 0,
    Ondra = 1,
    Kael = 2,
    Mirra = 3,
    Solun = 4,
}

impl CivId {
    /// All civs in island-index order.
    pub const ALL: [CivId; 5] = [
        CivId::Vesper,
        CivId::Ondra,
        CivId::Kael,
        CivId::Mirra,
        CivId::Solun,
    ];

    pub const fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(index: usize) -> Option<CivId> {
        Self::ALL.get(index).copied()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            CivId::Vesper => "vesper",
            CivId::Ondra => "ondra",
            CivId::Kael => "kael",
            CivId::Mirra => "mirra",
            CivId::Solun => "solun",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CivId;

    #[test]
    fn index_round_trips() {
        for (i, civ) in CivId::ALL.iter().enumerate() {
            assert_eq!(civ.index(), i);
            assert_eq!(CivId::from_index(i), Some(*civ));
        }
        assert_eq!(CivId::from_index(5), None);
    }
}
