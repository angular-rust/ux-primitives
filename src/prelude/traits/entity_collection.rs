use crate::prelude::Agenda;

use super::{AgendaManager, Entity};

/// The [EntityCollection] should be implemented by objects which compose multiple entities.
///
pub trait EntityCollection: AgendaManager {
    /// Adds an entity to this object's children.
    /// 
    /// # Arguments
    /// 
    /// * `entity` - The entity to add.
    /// * `agenda` - Assigns the entity to a specific agenda.  If none is specified will assign to Agenda::ALWAYS. (optional)
    /// * `is_added_to_view` - If true will add the child entity's view to this object's view. (optional, default: false)
    /// * `view_priority` - Sets the child entity's view stack priority order (higher numbers appear closer to the top of the stack). (optional, default: 0)
    /// 
    // TODO: Return Added entity (to allow decoration).  Or null if addition was unsuccessful.
    fn add_entity(
        &self,
        entity: Box<dyn Entity>,
        agenda: Option<Agenda>,
        is_added_to_view: Option<bool>,
        view_priority: Option<i32>,
    );
    // TODO: -> Box<dyn Entity>;

    /// Removes an entity from this object's children.
    /// # Arguments
    /// 
    /// * `entity` - The entity to remove.
    /// * `agenda` - If set then will only remove the specified entity from this agenda, else will remove from all agendas. (optional)
    /// * `is_removed_from_view` - If true the child entity's view will be removed from this object's view. (optional, default: false)
    /// 
    fn remove_entity(&self, entity: Box<dyn Entity>, agenda: Option<Agenda>, is_removed_from_view: Option<bool>);

    /// Retrieves all child entities.
    /// Return Array of matching entities.
    /// 
    /// Consider this a runtime only method, rather than calling it during constructor or initialization phases.
    /// 
    /// # Arguments
    /// 
    /// * `agenda` - Used to filter results to the specified agenda. (optional)
    /// 
    fn get_entities(&self, agenda: Option<Agenda>) -> Vec<Box<dyn Entity>>;

    // /// Retrieves all child entities that match type.
    // /// Return Array of matching entities.
    // /// 
    // /// Consider this a runtime only method, rather than calling it during constructor or initialization phases.
    // ///
    // /// # Arguments
    // /// 
    // /// * `classType` - The type of class to match (can be any class, type or interface).
    // /// * `agenda` - Used to filter results to the specified agenda.
    // /// * `is_bubble_down` - Set to true if you want to search this object's children for the requested entity. (optional, default: false)
    // /// * `is_bubble_up` - Set to true if you want to search this object's parent for the requested entity. (optional, default: false)
    // /// * `is_bubble_everywhere` - Set to true if you want to search the entire entity traversal stack for the requested entity. (optional, default: false)
    // /// 	
    // fn get_entities_by_class<T>(
    //     classType: Class<T>,
    //     agenda: Option<Agenda>,
    //     is_bubble_down: Option<bool>,
    //     is_bubble_up: Option<bool>,
    //     is_bubble_everywhere: Option<bool>,
    // ) -> Vec<T>;

    /// Retrieves the child entity with the specified id.
    /// Return The requested entity or null if no entity with this id was found.
    /// Consider this a runtime only method, rather than calling it during constructor or initialization phases.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The unique identifier of the entity you want to retrieve.
    /// * `agenda` - Used to filter results to the specified agenda.
    /// * `is_bubble_down` - Set to true if you want to search this object's children for the requested entity.
    /// * `is_bubble_up` - Set to true if you want to search this object's parent for the requested entity.
    /// * `is_bubble_everywhere` - Set to true if you want to search the entire entity traversal stack for the requested entity.
    /// 
    fn get_entity_by_id(
        &self,
        id: String,
        agenda: Option<Agenda>,
        is_bubble_down: Option<bool>,
        is_bubble_up: Option<bool>,
        is_bubble_everywhere: Option<bool>,
    ) -> Box<dyn Entity>;
}
