use std::ops::Deref;

use base::trace::init_trace;
use base::uow::unit_of_work::{Phase, UnitOfWork, UoW};

use crate::data::SomeMessage;

mod data;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    init_trace();
}

#[tokio::test]
async fn new_uow() {
    let uow = UnitOfWork::new(SomeMessage);
    let phase = uow.phase().deref().clone();

    assert_eq!(phase, Phase::NotStarted);
}

#[tokio::test]
async fn add_handler() {
    let mut uow = UnitOfWork::new(SomeMessage);
    let consumer = |_uow| {
        println!("on_rollback consumer");
    };

    uow.on_rollback(consumer);
}
