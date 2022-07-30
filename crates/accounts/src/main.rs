use anyhow::Result;
use tonic::{transport::Server, Request, Response, Status};
use tracing::info;

use ::accounts::util::setup_tracing;

use crate::accounts::accounts_server::{Accounts, AccountsServer};
use crate::accounts::{AccountType, CreateAccountReply, CreateAccountRequest};

pub mod accounts {
    tonic::include_proto!("accounts");
}

#[derive(Debug, Default)]
pub struct BorAccounts {}

#[tonic::async_trait]
impl Accounts for BorAccounts {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountReply>, Status> {
        info!("Got a request: {:?}", request);

        let reply = CreateAccountReply {
            account_type: AccountType::Savings.into(),
            user_id: "123".to_owned(),
            balance: 100,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();

    let addr = "[::0]:50051".parse()?;
    let greeter = BorAccounts::default();

    info!("Starting authorization server {}", &addr);

    Server::builder()
        .add_service(AccountsServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
