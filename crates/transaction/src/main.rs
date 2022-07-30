use anyhow::Result;
use tonic::{transport::Server, Request, Response, Status};

use crate::trx::transaction_service_server::{TransactionService, TransactionServiceServer};
use crate::trx::{RecordReply, RecordRequest};

pub mod trx {
    tonic::include_proto!("transaction");
}

#[derive(Debug, Default)]
pub struct BorTransactions {}

#[tonic::async_trait]
impl TransactionService for BorTransactions {
    async fn record_authorized(
        &self,
        request: Request<RecordRequest>,
    ) -> Result<Response<RecordReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = RecordReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::0]:50052".parse()?;
    let greeter = BorTransactions::default();

    println!("Starting transaction server {}", &addr);

    Server::builder()
        .add_service(TransactionServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
