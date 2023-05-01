# Unit of Work

```rust
use async_trait::async_trait;

use crate::correlation::CorrelationProvider;
use crate::ddd::metadata::MetaData;
use crate::message::consumer::Consumer;
use crate::message::Message;
use crate::transaction::{Transaction, TransactionManager};

///  Enum indicating possible phases of the Unit of Work.
pub enum Phase {
    ///  Indicates that the unit of work has been created but has not been registered
    /// with the `CurrentUnitOfWork`
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

/// Represents a Unit of Work that monitors the processing of a `Message`.
#[async_trait]
pub trait UnitOfWork<T: Message>: Send + Sync {
    type Error;

    /// Starts the current unit of work. The UnitOfWork instance is registered with the
    /// `CurrentUnitOfWork`.
    fn start(&mut self) -> Result<(), Self::Error>;

    /// Commits the Unit of Work. This should be invoked after the Unit of Work Message has been processed.
    /// Handlers registered to the Unit of Work will be notified.
    ///
    /// After the commit (successful or not), any registered clean-up handlers. will be
    /// invoked and the Unit of Work is deregistered from the `CurrentUnitOfWork`.
    ///
    /// If the Unit of Work fails to commit, e.g. because an error is raised by one of its
    /// handlers, the Unit of Work is rolled back.
    fn commit(&mut self) -> Result<(), Self::Error>;

    /// Initiates the rollback of this Unit of Work, invoking all registered rollback
    fn rollback(&mut self, reason: impl Into<String>) -> Result<(), Self::Error>;

    /// Indicates whether this UnitOfWork is started. It is started when the `start`
    /// method has been called, and if the UnitOfWork has not been committed or rolled back.
    fn is_active(&self) -> bool;

    /// Returns the current phase of the Unit of Work.
    fn phase(&self) -> &Phase;

    /// Register given `handler` with the Unit of Work. The handler will be notified when the
    /// phase of the Unit of Work changes to `Phase::PREPARE_COMMIT`.
    fn on_prepare_commit<U: UnitOfWork<T>, C: Consumer<U>>(
        &self,
        consumer: C,
    ) -> Result<(), Self::Error>;

    /// Register given `handler` with the Unit of Work. The handler will be notified when the phase of the
    /// Unit of Work changes to `Phase::COMMIT`.
    fn on_commit<U: UnitOfWork<T>, C: Consumer<U>>(&self, consumer: C) -> Result<(), Self::Error>;

    /// Register given `handler` with the Unit of Work. The handler will be notified when the phase of the
    /// Unit of Work changes to `Phase::AFTER_COMMIT`.
    fn after_commit<U: UnitOfWork<T>, C: Consumer<U>>(
        &self,
        consumer: C,
    ) -> Result<(), Self::Error>;

    /// Register given `handler` with the Unit of Work. The handler will be notified when the phase of the
    /// Unit of Work changes to `Phase::ROLLBACK`. On rollback, the cause for the rollback
    /// can obtained from the supplied
    fn on_rollback<U: UnitOfWork<T>, C: Consumer<U>>(&self, consumer: C)
                                                     -> Result<(), Self::Error>;

    /// Register given `handler` with the Unit of Work. The handler will be notified when the phase of the
    ///  Unit of Work changes to `Phase::CLEANUP`.
    fn on_cleanup<U: UnitOfWork<T>, C: Consumer<U>>(&self, consumer: C) -> Result<(), Self::Error>;

    /// Returns an optional for the parent of this Unit of Work. The optional holds the Unit of Work
    /// that was active when this Unit of Work was started. In case no other Unit of Work was
    /// active when this Unit of Work was started the option is None, indicating that this is
    /// the Unit of Work root.
    fn parent<U: UnitOfWork<T>>(&self) -> Option<U>;

    /// Check that returns `true` if this Unit of Work does not have a parent.
    fn is_root<U: UnitOfWork<T>>(&self) -> bool {
        self.parent::<U>().is_some()
    }

    /// Returns the root of this Unit of Work. If this Unit of Work has no parent `parent()` it
    /// return itself, otherwise it returns the root of its parent.
    fn root<U: UnitOfWork<T>>(&self) -> U;

    /// Get the message that is being processed by the Unit of Work. A Unit of Work processes a single Message over its
    /// life cycle.
    fn message(&self) -> &T;

    ///  Get the correlation data contained in the `message()` being processed by the
    /// Unit of Work.
    fn correlation_data(&self) -> &MetaData;

    /// Register given `correlation_data` with this Unit of Work. Correlation data providers are used
    /// to provide meta data based on this Unit of Work's `message()` Message when
    /// `correlation_data()` is invoked.
    fn register_correlation_provider<P: CorrelationProvider<T>>(&mut self, provider: P);

    /// Attach a transaction to this Unit of Work, using the given `trx_manager`. The transaction
    /// will be managed in the lifecycle of this Unit of Work. Failure to start a transaction
    /// will cause this Unit of Work to be rolled back.
    fn attach_transaction<Tx: Transaction, Tm: TransactionManager<Tx>>(&self, trx_manager: Tm);

    /// Execute the given task in the context of this Unit of Work. If the Unit of Work is not started yet
    /// it will be started.
    async fn execute<F, C: RollbackConfig>(
        &self,
        task: F,
        rollback: Option<C>,
    ) -> Result<(), Self::Error>;
}

///  The RollbackConfiguration defines if a Unit of Work should be rolled back when an exception is raised during the
/// processing of a Message.
pub trait RollbackConfig {
    type Error;

    /// Decides whether the given error should trigger a rollback.
    fn rollback_on(&self, error: Self::Error) -> bool;
}

pub struct DefaultUnitOfWork;

#[cfg(test)]
mod uow_test {
    use crate::message::consumer::Consumer;

    #[derive(Debug)]
    struct SomeMessage;

    #[derive(Debug)]
    struct SomeConsumer;

    impl Consumer<SomeMessage> for SomeConsumer {
        fn accept(self, _value: SomeMessage) -> Self {
            self
        }
    }

    #[derive(Debug)]
    struct AnotherConsumer;

    impl Consumer<SomeMessage> for AnotherConsumer {
        fn accept(self, value: SomeMessage) -> Self {
            println!("another consumer: {:?}", value);

            self
        }
    }

    #[test]
    fn test_create() {
        let event = SomeMessage;
        let consumer1 = SomeConsumer;
        let consumer2 = AnotherConsumer;

        let consumer = consumer1.accept(event);
        consumer.and_then(consumer2);
    }
}

```

## Implementation

```rust
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

```