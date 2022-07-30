use serde::Serialize;

use crate::aggregate::AccountType;

#[derive(Debug, Serialize)]
pub enum AccountCommand {
    Open {
        account_id: Option<String>,
        member_id: String,
        account_type: AccountType,
    },
    Close {
        account_id: String,
    },
}
