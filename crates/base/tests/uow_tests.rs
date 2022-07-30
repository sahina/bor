use crate::data::{SomeHandler, SomeMessage};
use base::uow::unit_of_work::{Phase, TransactionManager, UnitOfWork};

mod data;

#[test]
fn test_transaction_manager() {
    let trx = TransactionManager::new();
    let read = trx.phase.lock().unwrap();

    assert_eq!(*read, Phase::NotStarted);
}

#[tokio::test]
async fn test_uow() {
    let message = SomeMessage;
    let handler = SomeHandler;

    let mut uow = UnitOfWork::new(message, handler);

    uow.start().await;
}
