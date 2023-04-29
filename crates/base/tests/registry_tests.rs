use base::trace::init_trace;
use base::uow::registry::HandlerRegistry;

use crate::data::{SomeHandler, SomeMessage};

mod data;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    init_trace();
}

#[tokio::test]
async fn test_registry_handle_existing() {
    let mut registry = HandlerRegistry::new();
    registry.register(SomeMessage, SomeHandler);

    let result = registry.handle(&SomeMessage).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_registry_handle_not_existing() {
    let mut registry = HandlerRegistry::new();
    registry.register(SomeMessage, SomeHandler);

    let result = registry.handle(&SomeMessage).await;

    assert!(result.is_ok());
}
