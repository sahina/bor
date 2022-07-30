use crate::ddd::metadata::MetaData;
use crate::message::Message;

pub mod multi;
pub mod originator;
pub mod simple;

/// Object defining the data from a Message that should be attached as correlation data
/// to messages generated as result of the processing of that message.
pub trait CorrelationProvider<T: Message>: Send + Sync {
    /// Provides a dictionary with the entries to attach as correlation data to generated messages
    /// while processing the given message
    fn correlation_for(&self, message: T) -> MetaData;
}
