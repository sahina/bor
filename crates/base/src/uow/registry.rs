use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::sync::Arc;

use tracing::{error, trace};

use crate::message::handler::MessageHandler;
use crate::message::Message;

pub struct HandlerRegistry<T: Message, E: Error> {
    handlers: HashMap<T, Arc<dyn MessageHandler<T, E>>>,
}

#[allow(clippy::new_without_default)]
impl<T: Message + PartialEq + Eq + Hash, E: Error> HandlerRegistry<T, E> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<F: MessageHandler<T, E> + 'static>(&mut self, message: T, handler: F) {
        self.handlers.insert(message, Arc::new(handler));
    }

    pub async fn handle(&self, message: &T) -> Result<(), E> {
        match self.handlers.get(message) {
            None => {
                trace!("MessageHandler: not found");
                Ok(())
            }
            Some(handler) => {
                trace!("MessageHandler: found");
                match handler.handle(message).await {
                    Ok(_) => {
                        trace!("MessageHandler: handled message");
                        Ok(())
                    }
                    Err(e) => {
                        error!("MessageHandler: {:?}", e);
                        Err(e)
                    }
                }
            }
        }
    }
}
