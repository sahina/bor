use crate::ddd::entity::Entity;
use crate::ddd::metadata::MetaData;
use crate::message::payload::Payload;

pub mod consumer;
pub mod generic;
pub mod handler;
pub mod payload;
pub mod processor;

/// Representation of a Message, containing a Payload and MetaData. Typical examples of Messages are Commands, Events and
/// Queries.
pub trait Message: Send + Clone {
    /// Returns the identifier of this message. Two messages with the same identifiers should be interpreted as different
    /// representations of the same conceptual message. In such case, the meta-data may be
    /// different for both representations. The payload  be identical.
    fn identifier(&self) -> Entity;

    /// Returns the meta data for this message. This meta data is a collection of key-value pairs, where the key is a
    /// String, and the value is a serializable object.
    fn metadata(&self) -> MetaData;

    /// Returns the payload of this message. The payload is the application-specific information.
    fn payload(&self) -> Payload;
}
