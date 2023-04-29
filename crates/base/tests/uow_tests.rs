use base::trace::init_trace;
use base::uow::trx::{Phase, TransactionManager};
use base::uow::unit_of_work::UnitOfWork;

use crate::data::{SomeHandler, SomeMessage};

mod data;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    init_trace();
}

#[tokio::test]
async fn test_transaction_manager() {
    let trx = TransactionManager::new();
    let read = trx.phase.read().await;

    assert_eq!(*read, Phase::NotStarted);
}

#[tokio::test]
async fn test_uow() {
    let message = SomeMessage;
    let handler = SomeHandler;

    let mut uow = UnitOfWork::new(message, handler);

    uow.begin().await;
}
