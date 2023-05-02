use std::sync::{Arc, RwLock, RwLockReadGuard};

use async_trait::async_trait;

use crate::message::Message;

///  Enum indicating possible phases of the `UnitOfWork`.
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub enum Phase {
    #[default]
    NotStarted,
    Started,
    PrepareCommit,
    Commit,
    Rollback,
    AfterCommit,
    Cleanup,
    Closed,
}

#[async_trait]
pub trait UoW<T: Message> {
    /// Starts Unit of Work.
    fn start(&mut self);

    /// Current phase of the Unit of Work.
    fn phase(&self) -> RwLockReadGuard<Phase>;

    ///  Register given consumer with the Unit of Work. The handler will be notified when the
    /// phase of the Unit of Work changes to `Phase::ROLLBACK`. On rollback, the cause for the
    /// rollback can obtained from the supplied.
    fn on_rollback<F>(&mut self, consumer: F)
    where
        F: Fn(UnitOfWork<T>) + 'static;

    /// Initiates the rollback, invoking all registered rollback with `on_rollback`.
    fn rollback(&mut self, cause: impl Into<String>);

    /// Get the message that is being processed by the Unit of Work. A Unit of Work processes a
    /// single Message over its life cycle.
    fn message(&self) -> T;

    ///  Execute the given {@code task} in the context of this Unit of Work. If the Unit of Work is not started yet
    /// it will be started.
    /// If the task executes successfully the Unit of Work is committed. If any exception is
    /// raised while executing the task, the Unit of Work is rolled back and the exception is
    /// thrown.
    fn execute<F>(&self, task: F)
    where
        F: Fn();
}

/// Implementation of the UnitOfWork that processes a single message.
#[derive(Default)]
pub struct UnitOfWork<T: Message> {
    message: T,
    phase: RwLock<Phase>,
    consumers: Vec<Arc<dyn Fn(UnitOfWork<T>)>>,
    rollback: bool,
    cause: String,
}

#[async_trait]
impl<T: Message> UoW<T> for UnitOfWork<T> {
    fn start(&mut self) {
        self.rollback = false;
        *(self.phase.write().unwrap()) = Phase::Started;
    }

    fn phase(&self) -> RwLockReadGuard<Phase> {
        self.phase.read().unwrap()
    }

    fn on_rollback<F>(&mut self, consumer: F)
    where
        F: Fn(UnitOfWork<T>) + 'static,
    {
        self.consumers.push(Arc::new(consumer));
    }

    fn rollback(&mut self, cause: impl Into<String>) {
        self.rollback = true;
        self.cause = cause.into();

        // check phase active and before Rollback
        let phase = self.phase();
        if !((*phase as u32) > (Phase::Rollback as u32) && *phase == Phase::Started) {}

        // todo: log
        // todo: invoke consumers
    }

    fn message(&self) -> T {
        self.message.clone()
    }

    fn execute<F>(&self, _task: F)
    where
        F: Fn(),
    {
        todo!()
    }
}

impl<T: Message> UnitOfWork<T> {
    pub fn new(message: T) -> Self {
        UnitOfWork {
            message,
            phase: Default::default(),
            consumers: vec![],
            rollback: false,
            cause: Default::default(),
        }
    }

    pub async fn start_with(message: T) -> Self {
        let mut uow = UnitOfWork::new(message);
        uow.start();

        uow
    }
}
