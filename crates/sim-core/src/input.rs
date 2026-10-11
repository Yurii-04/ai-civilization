//! External inputs (blessings, admin actions), each stamped with a tick.

use crate::civ::CivId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Input {
    /// Tick at which the input is applied.
    pub tick: u64,
    pub civ: CivId,
    pub kind: InputKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum InputKind {
    /// Admin marker with no gameplay effect other than being mixed into the
    /// island state; used to test input timing. Blessings arrive in M6.
    Marker(u32),
}
