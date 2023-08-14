use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::command::CardCommand;
use crate::error::CardError;
use crate::event::CardEvent;
use crate::service::CardService;

#[derive(Default, Serialize, Deserialize)]
pub struct CardAggregate {}

#[async_trait]
impl Aggregate for CardAggregate {
    type Command = CardCommand;
    type Event = CardEvent;
    type Error = CardError;
    type Services = CardService;

    fn aggregate_type() -> String {
        "card".to_owned()
    }

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    fn apply(&mut self, event: Self::Event) {
        todo!()
    }
}
