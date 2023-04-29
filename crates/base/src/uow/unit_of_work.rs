use std::marker::PhantomData;

use tracing::{error, info, trace};

use crate::message::handler::MessageHandler;
use crate::message::Message;
use crate::uow::trx::{Phase, TransactionManager};

/// Implementation of the UnitOfWork that processes a single message.
#[derive(Debug, Default)]
pub struct UnitOfWork<T: Message + Clone, E, H: MessageHandler<T, E>> {
    message: T,
    transaction_manager: TransactionManager,
    handler: H,
    _phantom: PhantomData<E>,
}

impl<T: Message + Clone, E, H: MessageHandler<T, E>> UnitOfWork<T, E, H> {
    pub fn new(message: T, handler: H) -> Self {
        UnitOfWork {
            message,
            transaction_manager: TransactionManager::new(),
            handler,
            _phantom: Default::default(),
        }
    }

    pub async fn begin_with(message: T, handler: H) -> Self {
        let mut uow = UnitOfWork::new(message, handler);
        uow.begin().await;

        uow
    }

    pub async fn begin(&mut self) {
        trace!("uow: beginning");
        self.execute().await;
    }

    async fn execute(&mut self) {
        let read_phase = self.transaction_manager.phase.read().await;
        let mut can_start = false;
        if *read_phase == Phase::NotStarted {
            trace!("uow: phase Phase::NotStarted ");
            can_start = true;
        };
        drop(read_phase);

        if can_start {
            self.transaction_manager.start().await;
        }

        trace!("uow: executing");
        self.transaction_manager.start().await;

        let message = &self.message;

        // todo: pre-commit funcs
        self.transaction_manager.prepare_commit().await;

        let result = self.handler.handle(message).await;
        match result {
            Ok(_) => {
                // success
                info!("uow: handle:success");
                self.transaction_manager.commit().await;
                trace!("uow: commited");

                // todo: post-commit funcs
                self.transaction_manager.after_commit().await;
                trace!("uow: post_commited");
            }
            Err(_) => {
                error!("uow: handle:error");
                self.transaction_manager.rollback().await;
                trace!("uow: rolled back");
            }
        }

        // todo: cleanup funcs
        self.transaction_manager.after_cleanup().await;
        trace!("uow: cleaned up");
    }
}
