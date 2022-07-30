use async_trait::async_trait;
use serde::Serialize;
use tracing::{info, instrument, trace_span, warn};
use uuid::Uuid;

use base::ddd::aggregate::{Aggregate, Versioned};

use crate::command::AccountCommand;
use crate::error::Error;
use crate::event::AccountEvent;

// -- Aggregate Types

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub enum AccountType {
    #[default]
    Checking,
    Savings,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub enum AccountStatus {
    #[default]
    Open,
    Closed,
}

// -- Aggregate State

#[derive(Debug, Default)]
struct AccountState {
    id: String,
    account_type: AccountType,
    member_id: String,
    version: usize,
    status: AccountStatus,
}

impl Versioned for AccountState {
    fn version(&self) -> usize {
        self.version
    }
}

// -- Aggregate

#[derive(Debug, Default)]
struct Account {
    state: AccountState,
}

#[async_trait]
impl Aggregate for Account {
    type Error = Error;
    type State = AccountState;
    type Command = AccountCommand;
    type Event = AccountEvent;
    type Service = ();

    #[instrument(skip(self))]
    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Service,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            AccountCommand::Open {
                account_id,
                member_id,
                account_type,
            } => {
                let span = trace_span!("command.validation");
                let entry = span.enter();
                if member_id.is_empty() {
                    warn!("Member id missing -> `AccountCommand::Open`");
                    return Err(Error::AccountMissingMemberId);
                }
                drop(entry);

                info!("Account Opened -> id: `{account_id:?}`");

                Ok(vec![AccountEvent::Opened {
                    account_id: account_id.unwrap_or(Uuid::new_v4().to_string()),
                    member_id,
                    account_type,
                }])
            }
            AccountCommand::Close { account_id } => {
                let mut events = vec![];

                let span = trace_span!("command.validation");
                let entry = span.enter();
                if self.state.status != AccountStatus::Open {
                    warn!("Only open accounts can be closed");
                } else {
                    events.push(AccountEvent::Closed { id: account_id });
                }
                drop(entry);

                Ok(events)
            }
        }
    }

    #[instrument]
    fn apply(mut self, event: Self::Event) -> Self
    where
        Self: Sized,
    {
        match event {
            AccountEvent::Opened {
                account_id: id,
                member_id,
                account_type,
            } => {
                self.state.id = id;
                self.state.member_id = member_id;
                self.state.account_type = account_type;
                self.state.status = AccountStatus::Open;
            }
            AccountEvent::Closed { .. } => self.state.status = AccountStatus::Closed,
        }

        self.state.version += 1;
        self
    }
}

#[cfg(test)]
mod account_test {
    use tracing_test::traced_test;

    use base::tester::framework::TestFramework;

    use crate::aggregate::{Account, AccountType};
    use crate::command::AccountCommand;
    use crate::error::Error;
    use crate::event::AccountEvent;

    #[test]
    #[traced_test]
    fn open_account() {
        let command = AccountCommand::Open {
            account_id: Some("1".to_owned()),
            member_id: "1".to_owned(),
            account_type: AccountType::Savings,
        };
        let expected: Vec<AccountEvent> = vec![AccountEvent::Opened {
            account_id: "1".to_owned(),
            member_id: "1".to_owned(),
            account_type: AccountType::Savings,
        }];

        TestFramework::<Account>::with(())
            .given_no_previous_events()
            .when(command)
            .then_expect_events(expected);
    }

    #[test]
    #[traced_test]
    fn open_account_missing_member_id_() {
        let command = AccountCommand::Open {
            account_id: Some("1".to_owned()),
            member_id: "".to_owned(),
            account_type: AccountType::Savings,
        };

        TestFramework::<Account>::with(())
            .given_no_previous_events()
            .when(command)
            .then_expect_error(Error::AccountMissingMemberId);
    }

    #[test]
    #[traced_test]
    fn test_close_account() {
        let previous_events = vec![AccountEvent::Opened {
            account_id: "1".to_owned(),
            member_id: Default::default(),
            account_type: Default::default(),
        }];
        let expected_events = vec![AccountEvent::Closed {
            id: "1".to_string(),
        }];

        TestFramework::<Account>::with(())
            .given(previous_events)
            .when(AccountCommand::Close {
                account_id: "1".to_string(),
            })
            .then_expect_events(expected_events);
    }

    #[test]
    #[traced_test]
    fn test_close_closed_account() {
        let previous_events = vec![
            AccountEvent::Opened {
                account_id: "1".to_owned(),
                member_id: Default::default(),
                account_type: Default::default(),
            },
            AccountEvent::Closed {
                id: "1".to_string(),
            },
        ];
        let expected_events = vec![];

        TestFramework::<Account>::with(())
            .given(previous_events)
            .when(AccountCommand::Close {
                account_id: "1".to_string(),
            })
            .then_expect_events(expected_events);
    }
}
