use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use tracing::{error, info, trace};

use crate::message::handler::MessageHandler;
use crate::message::Message;

///  Enum indicating possible phases of the Unit of Work.
#[derive(Debug, Default, Eq, PartialEq)]
pub enum Phase {
    ///  Indicates that the unit of work has been created but has not been registered
    /// with the `CurrentUnitOfWork`
    #[default]
    NotStarted,
    /// Indicates that the Unit of Work has been registered with the `CurrentUnitOfWork` but has not been
    //  committed, because its Message has not been processed yet.
    Started,
    /// Indicates that the Unit of Work is preparing its commit. This means that `commit` has been
    /// invoked on the Unit of Work, indicating that the Message of the Unit of Work has been
    /// processed. All handlers registered to be notified before commit  will be invoked. If no
    /// error is raised by any of the handlers the Unit of Work will go into the phase,
    /// otherwise it will be rolled back.
    PrepareCommit,
    /// Indicates that the Unit of Work has been committed and is passed the `PrepareCommit`  phase.
    Commit,
    /// Indicates that the Unit of Work is being rolled back. Generally this is because an error
    /// was raised while processing the message or while the Unit of Work was being committed.
    Rollback,
    /// Indicates that the Unit of Work is after a successful commit. In this phase the Unit of Work cannot be rolled
    ///  back anymore.
    AfterCommit,
    /// Indicates that the Unit of Work is after a successful commit or after a rollback. Any resources tied to this
    /// Unit of Work should be released.
    Cleanup,
    /// Indicates that the Unit of Work is at the end of its life cycle. This phase is final.
    Closed,
}

#[derive(Debug, Default)]
pub struct TransactionManager {
    pub phase: Arc<Mutex<Phase>>,
}

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager::default()
    }

    pub fn start(&self) {
        let mut phase = self.phase.lock().unwrap();
        *phase = Phase::Started;
    }

    pub fn commit(&self) {
        let mut phase = self.phase.lock().unwrap();
        *phase = Phase::Commit;
    }

    pub fn prepare_commit(&self) {
        let mut phase = self.phase.lock().unwrap();
        *phase = Phase::PrepareCommit;
    }

    pub fn after_commit(&self) {
        let mut phase = self.phase.lock().unwrap();
        *phase = Phase::AfterCommit;
    }

    pub fn after_cleanup(&self) {
        let mut phase = self.phase.lock().unwrap();
        *phase = Phase::Cleanup;
    }

    pub fn after_closed(&self) {
        let mut phase = self.phase.lock().unwrap();
        *phase = Phase::Closed;
    }

    pub fn rollback(&self) {
        let mut phase = self.phase.lock().unwrap();
        *phase = Phase::Rollback;
    }
}

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

    pub async fn start(&mut self) {
        trace!("uow: starting");
        self.execute().await;
    }

    async fn execute(&mut self) {
        trace!("uow: executing");
        self.transaction_manager.start();

        let message = &self.message;

        // todo: pre-process funcs
        self.transaction_manager.prepare_commit();

        let result = self.handler.handle(message).await;
        match result {
            Ok(_) => {
                // success
                info!("uow: handle:success");
                self.transaction_manager.commit();
                trace!("uow: commited");

                // todo: post-commit funcs
                self.transaction_manager.after_commit();
                trace!("uow: post_commited");
            }
            Err(_) => {
                error!("uow: handle:error");
                self.transaction_manager.rollback();
                trace!("uow: rolled back");
            }
        }

        // todo: cleanup funcs
        self.transaction_manager.after_cleanup();
        trace!("uow: cleaned up");
    }
}
