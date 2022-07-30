#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use async_trait::async_trait;

pub trait Event: 'static + Eq + Hash + Send + Sync {}

pub trait Command: 'static + Send + Sync {}

#[async_trait]
pub trait EventHandler<E: Event, C: Command> {
    async fn handle(&self, event: E) -> C;
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum SagaState {
    #[default]
    Created,
    Started,
    Cancelled,
    Finalized,
}

#[derive(Default)]
pub struct Saga<E, C> {
    state: SagaState,
    handlers: HashMap<E, Arc<dyn EventHandler<E, C>>>,
}

impl<E, C> Saga<E, C>
where
    E: Event,
    C: Command,
{
    pub fn new() -> Self {
        Saga {
            state: SagaState::Created,
            handlers: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        self.state = SagaState::Started;
    }

    pub fn cancel(&mut self) {
        self.state = SagaState::Cancelled;
    }

    pub fn finalize(&mut self) {
        self.state = SagaState::Finalized;
    }

    pub fn register_function<F>(&mut self, event: E, event_handler: F)
    where
        F: EventHandler<E, C> + 'static,
    {
        self.handlers.insert(event, Arc::new(event_handler));
    }

    pub async fn handle(&mut self, event: E) -> Option<C> {
        if self.state == SagaState::Started {
            if let Some(handler) = self.handlers.get(&event) {
                Some(handler.handle(event).await)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tester {
    use async_trait::async_trait;

    use crate::saga::orchestrate::{Command, Event, EventHandler, Saga};

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum MyEvent {
        Created { value: i32 },
        Incremented { value: i32 },
    }

    impl Event for MyEvent {}

    #[derive(Debug)]
    enum MyCommand {
        Create { value: i32 },
        Increment { value: i32 },
    }

    impl Command for MyCommand {}

    struct MyEventHandler;

    #[async_trait]
    impl EventHandler<MyEvent, MyCommand> for MyEventHandler {
        async fn handle(&self, event: MyEvent) -> MyCommand {
            match event {
                MyEvent::Created { value } => MyCommand::Create { value },
                MyEvent::Incremented { value } => MyCommand::Increment { value },
            }
        }
    }

    #[tokio::test]
    async fn run() {
        let mut saga = Saga::<MyEvent, MyCommand>::new();
        saga.start();

        let event1 = MyEvent::Created { value: 42 };
        let event2 = MyEvent::Incremented { value: 42 };

        saga.register_function(event1.clone(), MyEventHandler);
        saga.register_function(event2.clone(), MyEventHandler);

        let command1 = saga.handle(event1).await;
        let command2 = saga.handle(event2).await;

        assert!(command1.is_some());
        assert!(command2.is_some());
    }
}
