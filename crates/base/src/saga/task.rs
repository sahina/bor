use std::sync::Arc;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Error executing transaction")]
    Action,
}

/// Task is a combination of a Transaction function `T` and a Compensating function `C`.
///
/// If Transaction `T` is executed successfully, Compensating function `C` will not be
/// executed.
#[derive(Debug, Clone)]
pub struct Task<T, C>
where
    T: Fn() -> Result<(), ()> + Send + 'static,
    C: Fn() + Send + Sync,
{
    /// State of the action
    state: TaskState,
    /// Action function
    action: Arc<T>,
    /// Compensating function    
    compensation: Arc<C>,
}

impl<T, C> Task<T, C>
where
    T: Fn() -> Result<(), ()> + Send + 'static,
    C: Fn() + Send + Sync,
{
    pub fn new(tx: T, cx: C) -> Self {
        Self {
            state: TaskState::ScheduledRun,
            action: Arc::new(tx),
            compensation: Arc::new(cx),
        }
    }

    pub async fn start(&mut self) -> Result<&Self, TaskError> {
        self.state = TaskState::Running;
        let result = run_function(self.action.clone()).await;

        match result {
            Ok(_) => {
                self.state = TaskState::Finished;
            }
            Err(_) => {
                self.state = TaskState::Aborting;
                (self.compensation)();
                self.state = TaskState::Aborted;
            }
        }

        self.state = TaskState::Finished;

        Ok(self)
    }

    pub fn abort(&mut self) {
        self.state = TaskState::Aborted;
    }
}

async fn run_function<F>(func: Arc<F>) -> Result<(), TaskError>
where
    F: Fn() -> Result<(), ()> + Send + 'static,
{
    let result = func();

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(TaskError::Action),
    }
}

/// State of action
#[derive(Debug, Clone)]
pub enum TaskState {
    ScheduledRun,
    ScheduledAbort,
    Running,
    Aborting,
    Finished,
    Aborted,
}

#[cfg(test)]
mod task_test {
    #[tokio::test]
    async fn test_simple_task() {}
}
