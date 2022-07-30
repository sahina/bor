use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::ddd::constants::{ENTITY_ID_KEY, ENTITY_KEY, ENTITY_NAME_KEY, EVENT_NAME_KEY};
use crate::ddd::entity::Entity;
use crate::ddd::metadata::MetaData;
use crate::message::payload::Payload;
use crate::message::Message;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    identifier: Entity,
    metadata: MetaData,
    payload: Payload,
}

impl EventMessage {
    pub fn new(event_name: impl Into<String>, payload: impl Into<Value>) -> Self {
        let identifier = Entity::from_name(event_name);
        // todo: may be store only entity as identifier
        let metadata = MetaData::new()
            .insert(EVENT_NAME_KEY, identifier.name())
            .insert(ENTITY_KEY, json!(identifier))
            .insert(ENTITY_ID_KEY, identifier.id())
            .insert(ENTITY_NAME_KEY, identifier.name());

        EventMessage {
            identifier,
            metadata,
            payload: Payload::new(payload),
        }
    }

    pub fn set_identifier(mut self, identifier: impl Into<Entity>) -> Self {
        self.identifier = identifier.into();
        self
    }

    pub fn add_meta(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.metadata.add(key.into(), value);
        self
    }
}

impl Message for EventMessage {
    fn identifier(&self) -> Entity {
        self.identifier.clone()
    }

    fn metadata(&self) -> MetaData {
        self.metadata.clone()
    }

    fn payload(&self) -> Payload {
        self.payload.clone()
    }
}

#[cfg(test)]
mod message_test {
    use crate::ddd::entity::Entity;
    use crate::message::generic::EventMessage;
    use crate::message::payload::Payload;
    use crate::message::Message;

    #[test]
    fn build_event_message() {
        let expected_id = Entity::new("id", "name");
        let expected_payload = Payload::new("my payload");

        let event_message =
            EventMessage::new("some_event", "my payload").set_identifier(expected_id.clone());

        assert_eq!(event_message.identifier(), expected_id);
        assert_eq!(event_message.payload(), expected_payload);
    }
}
