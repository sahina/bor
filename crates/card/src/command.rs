use crate::payload::{
    ActivateCardPayload, DeactivateCardPayload, IssueCardPayload, LoseCardPayload,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CardCommand {
    Issue(IssueCardPayload),
    Activate(ActivateCardPayload),
    Deactivate(DeactivateCardPayload),
    Lose(LoseCardPayload),
}
