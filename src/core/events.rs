use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::broadcast;

/// System-wide events emitted by Oxide_CG engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    RecordCreated { model: String, id: i64, data: Value },
    RecordUpdated { model: String, id: i64, changes: Value },
    RecordDeleted { model: String, id: i64 },
    ApprovalRequested { approval_id: i64, model: String, record_id: i64 },
    ApprovalResolved { approval_id: i64, approved: bool },
    RollbackExecuted { log_id: i64, model: String, record_id: i64 },
}

/// Asynchronous, broadcast-capable event bus for Oxide_CG
pub struct EventBus {
    sender: broadcast::Sender<SystemEvent>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Publish an event to all subscribers without blocking
    pub fn publish(&self, event: SystemEvent) {
        let _ = self.sender.send(event);
    }

    /// Subscribe to system events
    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.sender.subscribe()
    }
}
