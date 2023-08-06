use async_trait::async_trait;

use crate::message::Message;

/// Component that processes Messages.
#[async_trait]
pub trait MessageProcessor<T: Message, E, R>: Send + Sync {
    /// Processes the given message.
    async fn process(&self, message: &T) -> Result<R, E>;
}
