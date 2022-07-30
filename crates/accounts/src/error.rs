use serde::Serialize;
use strum_macros::AsRefStr;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Serialize, AsRefStr, Error)]
pub enum Error {
    // -- Account errors
    #[error("Account is missing member id")]
    AccountMissingMemberId,
}
