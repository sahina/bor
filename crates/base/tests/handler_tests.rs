use base::message::handler::MessageHandler;
use base::trace::init_trace;

use crate::data::{SomeHandler, SomeHandlerInError, SomeMessage};

mod data;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    init_trace();
}

#[tokio::test]
async fn test_message_handler() {
    let handler = SomeHandler;
    let result = handler.handle(&SomeMessage).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_message_handler_with_error() {
    let handler = SomeHandlerInError;
    let result = handler.handle(&SomeMessage).await;

    assert!(result.is_err());
}
