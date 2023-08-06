use async_trait::async_trait;

use crate::message::Message;

/// Component that handles Messages.
#[async_trait]
pub trait MessageHandler<T: Message, E>: Send + Sync {
    /// Handles the given message.
    async fn handle(&self, message: &T) -> Result<(), E>;
}
