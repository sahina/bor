use crate::payload::{
    CardActivatedPayload, CardDeactivatePayload, CardIssuedPayload, CardLostPayload,
};
use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CardEvent {
    Issued(CardIssuedPayload),
    Activated(CardActivatedPayload),
    Deactivated(CardDeactivatePayload),
    Lost(CardLostPayload),
}

impl DomainEvent for CardEvent {
    fn event_type(&self) -> String {
        match self {
            CardEvent::Issued(_) => "Issued".to_owned(),
            CardEvent::Activated(_) => "Activated".to_owned(),
            CardEvent::Deactivated(_) => "Deactivated".to_owned(),
            CardEvent::Lost(_) => "Lost".to_owned(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_owned()
    }
}
