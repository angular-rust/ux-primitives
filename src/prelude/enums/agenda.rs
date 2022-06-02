use std::any::TypeId;

/// Can be used to influence the internal state of an `Entity` via `AgendaManager`.
/// 
/// Can be extended with Custom by using concrete project values.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agenda {
    /// The default Agenda.  Anything assigned to this will be run each update irrespective of what agenda the parent is assigned.
    Always,
    /// The Birth Agenda
    Birth,
    /// The Death Agenda
    Death,
    /// The Standard Agenda
    Standard,
    /// The Attack Agenda
    Attack,
    /// The Defend Agenda
    Defend,

    /// Allows Agenda to be extended (e.g. for using entity specific enumerated agendas).
    Custom {
        /// Custom value
        value: TypeId,
    },
}
