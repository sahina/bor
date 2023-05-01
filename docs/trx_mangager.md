# Transaction Manager

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::trace;

///  Enum indicating possible phases of the `TransactionManager`.
#[derive(Debug, Default, Eq, PartialEq)]
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

#[derive(Debug, Default)]
pub struct TransactionManager {
    pub phase: Arc<RwLock<Phase>>,
}

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager::default()
    }

    pub async fn start(&self) {
        trace!("trx_mgr: start");
        let mut phase = self.phase.write().await;
        *phase = Phase::Started;
    }

    pub async fn commit(&self) {
        trace!("trx_mgr: commit");
        let mut phase = self.phase.write().await;
        *phase = Phase::Commit;
    }

    pub async fn prepare_commit(&self) {
        trace!("trx_mgr: prepare_commit");
        let mut phase = self.phase.write().await;
        *phase = Phase::PrepareCommit;
    }

    pub async fn after_commit(&self) {
        trace!("trx_mgr: after_commit");
        let mut phase = self.phase.write().await;
        *phase = Phase::AfterCommit;
    }

    pub async fn after_cleanup(&self) {
        trace!("trx_mgr: after_cleanup");
        let mut phase = self.phase.write().await;
        *phase = Phase::Cleanup;
    }

    pub async fn after_closed(&self) {
        trace!("trx_mgr: after_closed");
        let mut phase = self.phase.write().await;
        *phase = Phase::Closed;
    }

    pub async fn rollback(&self) {
        trace!("trx_mgr: rollback");
        let mut phase = self.phase.write().await;
        *phase = Phase::Rollback;
    }
}

```