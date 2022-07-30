use anyhow::Result;
use tonic::{transport::Server, Request, Response, Status};

use crate::authorizer::authorizer_server::{Authorizer, AuthorizerServer};
use crate::authorizer::{AuthReply, AuthRequest};

pub mod authorizer {
    tonic::include_proto!("auth");
}

#[derive(Debug, Default)]
pub struct BorAuthorizer {}

#[tonic::async_trait]
impl Authorizer for BorAuthorizer {
    async fn authorize_transaction(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = AuthReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::0]:50051".parse()?;
    let greeter = BorAuthorizer::default();

    println!("Starting authorization server {}", &addr);

    Server::builder()
        .add_service(AuthorizerServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
