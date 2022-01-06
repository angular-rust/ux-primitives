use crate::prelude::Agenda;

/// Objects intending to provide an [Agenda] driven state machine should implement this interface.
///
pub trait AgendaManager {
    /// The object's current agenda.
    fn agenda(&self) -> Agenda;

    /// Change the object's current agenda.
    /// Returns true if the agenda has changed, false otherwise.
    /// 
    /// # Arguments
    /// 
    /// * `val` - The new agenda.  If none is specified will assign to Agenda::ALWAYS.
    /// 
    fn set_egenda(&self, val: Agenda) -> bool;
}
