//! Events returned from [`crate::step`]; the source of the chronicle.

use crate::civ::CivId;
use crate::input::InputKind;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Event {
    /// A new sim day began (emitted once per world, not per island).
    DayStarted { day: u64 },
    /// An input was applied to an island at this tick.
    InputApplied { civ: CivId, kind: InputKind },
}
