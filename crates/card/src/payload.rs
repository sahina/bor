use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CardType {
    Platinum,
    Cash,
    Green,
}

//
// Command payloads

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueCardPayload {
    pub id: String,
    pub typ: CardType,
    pub number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateCardPayload {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeactivateCardPayload {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoseCardPayload {
    pub id: String,
}

//
// Event payloads

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardIssuedPayload {
    pub id: String,
    pub typ: CardType,
    pub number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardActivatedPayload {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardDeactivatePayload {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardLostPayload {
    pub id: String,
}
