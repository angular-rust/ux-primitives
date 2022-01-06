use std::any::TypeId;

/// Can be used to influence the internal state of an `Entity` via `AgendaManager`.
/// 
/// Can be extended with Custom by using concrete project values.
///
pub enum Agenda {
    /// The default Agenda.  Anything assigned to this will be run each update irrespective of what agenda the parent is assigned.
    Always,
    Birth,
    Death,
    Standard,
    Attack,
    Defend,

    /// Allows Agenda to be extended (e.g. for using entity specific enumerated agendas).
    Custom {
        value: TypeId,
    },
}
