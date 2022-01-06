use std::any::TypeId;

use super::{Entity, Resettable};

/// The MessageManager should be implemented by objects intending to fulfill Entity to Entity synchronous messaging (also known as events or signals).
/// 
/// The interface provides an observer pattern oriented manager allowing any [Entity} to listen to anything on any other [Entity].
/// Note, the author is not a fan of observer pattern and provides this manager with a note of caution - there is <i>always</i> a better way to communicate than to fire shots into the dark!
/// This manager is intentionally abstract / generic.  It allows expressive synchronous events - i.e. use anything as a message (string, enumerator, class, state based object etc).
/// It may make more sense to handle events using an alternative, event or signal specific library.  Adapt one as an [Entity] and inject into any scene as needed.
pub trait MessageManager<M>: Resettable {
    /// Register an entity's interest in a subject.
    /// 
    /// # Arguments
    /// 
    /// * `subscriber` - Entity listening / observing for messages.
    /// * `message` - Specific message to listen for.
    /// * `handler` - Function to pass observed messages to: receives Message & Sender and returns true if send propogation is to continue (true should be default behavior).
    /// * `sender` - Only listen to messages from this entity. (optional)
    /// * `senderClassType` - Only listen to messages from this type of entity. (optional)
    /// * `isRemovedAfterFirstSend` - Once a message has been received, no longer listen for further messages under the same criteria. (optional, default: false)
    /// @type <M> Messages can be any type: String, Class, Enum.  For recursive types use Enums.
    /// 
    fn add_subscriber(
        &self,
        subscriber: Box<dyn Entity>,
        message: M,
        handler: Box<dyn Fn(M, Box<dyn Entity>) -> bool>,
        sender: Option<Box<dyn Entity>>,
        sender_class: Option<TypeId>, // Option<Class<Entity>>
        is_removed_after_first_send: Option<bool>,
    );

    /// Retrieve all entity's interested in a subject.
    /// All parameters are optional to allow wildcard filtering.
    /// 
    /// # Arguments
    /// 
    /// * `subscriber` - Entity listening / observing for messages. (optional)
    /// * `message` - Specific message to listen for. (optional)
    /// * `handler` - Function to pass observed messages to. (optional)
    /// * `sender` - Only listen to messages from this entity. (optional)
    /// * `senderClassType` - Only listen to messages from this type of entity. (optional)
    ///
    /// Return: An array of entities corresponding to the specified filters.
    /// @type <M> Messages can be any type: String, Class, Enum.  For recursive types use Enums.
    /// 
    fn get_subscribers(
        &self,
        subscriber: Option<Box<dyn Entity>>,
        message: Option<M>,
        handler: Box<dyn Fn(M, Box<dyn Entity>) -> bool>,
        sender: Option<Box<dyn Entity>>,
        sender_class: Option<TypeId>, // Option<Class<Entity>>
    ) -> Vec<Box<dyn Entity>>;

    /// Unsubscribes entities matching the specified criteria.
    /// 
    /// # Arguments
    /// 
    /// * `subscriber` - Entity listening / observing for messages. (optional)
    /// * `message` - Specific message to listen for. (optional)
    /// * `handler` - Function to pass observed messages to. (optional)
    /// * `sender` - Only listen to messages from this entity. (optional)
    /// * `senderClassType` - Only listen to messages from this type of entity. (optional)
    /// @type <M> Messages can be any type: String, Class, Enum.  For recursive types use Enums.
    /// 
    fn remove_subscribers(
        &self,
        subscriber: Option<Box<dyn Entity>>,
        message: Option<M>,
        handler: Box<dyn Fn(M, Box<dyn Entity>) -> bool>,
        sender: Option<Box<dyn Entity>>,
        sender_class: Option<TypeId>, // Option<Class<Entity>>
    );

    /// Dispatch a message from a specific entity.
    /// 
    /// # Arguments
    /// 
    /// * `message` - Message to dispatch.
    /// * `sender` - The originator of the message (can be spoofed).
    /// * `isBubbleDown` - Set to true if you want to dispatch this message to the sender's children. (optional, default: false)
    /// * `isBubbleUp` - Set to true if you want to dispatch this message to the sender's parent. (optional, default: false)
    /// * `isBubbleEverywhere` - Set to true if you want to dispatch this message to the entity traversal stack. (optional, default: false)
    /// @type <M> Messages can be any type: String, Class, Enum.  For recursive types use Enums.
    /// 
    fn send(
        &self,
        message: M,
        sender: Box<dyn Entity>,
        is_bubble_down: Option<bool>,
        is_bubble_up: Option<bool>,
        is_bubble_everywhere: Option<bool>,
    );
}
