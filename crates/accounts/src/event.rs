use base::ddd::event::DomainEvent;
use serde::Serialize;

use crate::aggregate::AccountType;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AccountEvent {
    Opened {
        account_id: String,
        member_id: String,
        account_type: AccountType,
    },
    Closed {
        id: String,
    },
}

impl DomainEvent for AccountEvent {}
