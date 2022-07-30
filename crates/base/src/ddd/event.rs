use std::fmt::Debug;

/// Domain event generated within the bounded context of the ddd.
///
/// When domain events are sent across the network, they are sent inside a message.
///
/// In short, domain event is not an event message.
pub trait DomainEvent: Debug + Clone + PartialEq {}
