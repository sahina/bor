use std::convert::Infallible;

use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tracing::{error, info};

use base::ddd::constants::ENTITY_KEY;
use base::ddd::entity::Entity;
use base::ddd::event::DomainEvent;
use base::ddd::metadata::MetaData;
use base::message::handler::MessageHandler;
use base::message::payload::Payload;
use base::message::Message;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SomeEvent;

impl DomainEvent for SomeEvent {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SomeMessage;

impl Message for SomeMessage {
    fn identifier(&self) -> Entity {
        Entity::from_name("SomeMessage")
    }

    fn metadata(&self) -> MetaData {
        MetaData::new().insert(ENTITY_KEY, json!(self.identifier()))
    }

    fn payload(&self) -> Payload {
        Payload::new(json!(SomeEvent {}))
    }
}

pub struct SomeHandler;

#[async_trait]
impl MessageHandler<SomeMessage, Infallible> for SomeHandler {
    async fn handle(&self, message: &SomeMessage) -> Result<(), Infallible> {
        info!("handling {:?}", message);

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("oops")]
    Oops,
}

pub struct SomeHandlerInError;

#[async_trait]
impl MessageHandler<SomeMessage, HandlerError> for SomeHandlerInError {
    async fn handle(&self, message: &SomeMessage) -> Result<(), HandlerError> {
        error!("handling error: {:?}", message);

        Err(HandlerError::Oops)
    }
}
