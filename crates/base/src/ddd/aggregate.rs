use std::error::Error;

use async_trait::async_trait;

use crate::ddd::event::DomainEvent;

/// Entities can track changes with incrementing versions when changed.
pub trait Versioned {
    /// Gets current version
    fn version(&self) -> usize;
}

#[async_trait]
pub trait Aggregate: Default {
    type Error: Error;
    type State: Versioned;
    type Command;
    type Event: DomainEvent;
    type Service: Send + Sync;

    /// Handles given command.
    ///
    /// Business validation is done in this method.
    /// Returns events as result for the handled command.
    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Service,
    ) -> Result<Vec<Self::Event>, Self::Error>;

    /// Applies event to change state of aggregate.
    ///
    /// Validation should not be done in this method.
    /// Returns revised `Aggregate`. Consumes and returns owned. This
    /// avoids mutation or new allocation for event application.
    fn apply(self, event: Self::Event) -> Self;
}
