use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast;

/// Minimal lifecycle event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    /// Emitted when an app is compiled (for source map capture)
    AppCompiled,
    /// Emitted when a transaction group is simulated (for AVM traces)
    TxnGroupSimulated,
}

/// Minimal event payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppCompiledEventData {
    pub app_name: Option<String>,
    pub approval_source_map: Option<serde_json::Value>,
    pub clear_source_map: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxnGroupSimulatedEventData {
    pub simulate_response: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum EventData {
    AppCompiled(AppCompiledEventData),
    TxnGroupSimulated(TxnGroupSimulatedEventData),
}

/// Async event emitter using Tokio broadcast
#[derive(Clone)]
pub struct AsyncEventEmitter {
    sender: broadcast::Sender<(EventType, EventData)>,
}

impl AsyncEventEmitter {
    pub fn new(buffer: usize) -> Self {
        let (sender, _receiver) = broadcast::channel(buffer);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<(EventType, EventData)> {
        self.sender.subscribe()
    }

    pub async fn emit(&self, event_type: EventType, data: EventData) {
        // Ignore error if there are no subscribers
        let _ = self.sender.send((event_type, data));
    }
}

/// Global flags and event emitter
static DEBUG: AtomicBool = AtomicBool::new(false);
static TRACE_ALL: AtomicBool = AtomicBool::new(false);
static EVENTS: Lazy<AsyncEventEmitter> = Lazy::new(|| AsyncEventEmitter::new(32));

/// Global runtime config singleton
pub struct Config;

impl Config {
    pub fn debug() -> bool {
        DEBUG.load(Ordering::Relaxed)
    }

    pub fn trace_all() -> bool {
        TRACE_ALL.load(Ordering::Relaxed)
    }

    pub fn events() -> AsyncEventEmitter {
        EVENTS.clone()
    }

    pub fn configure(new_debug: Option<bool>, new_trace_all: Option<bool>) {
        if let Some(d) = new_debug {
            DEBUG.store(d, Ordering::Relaxed);
        }
        if let Some(t) = new_trace_all {
            TRACE_ALL.store(t, Ordering::Relaxed);
        }
    }
}
