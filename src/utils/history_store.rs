use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const HISTORY_KEY: &str = "portolio.history";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStore {
    history: Vec<String>,
}

impl HistoryStore {
    pub fn new() -> Self {
        if let Ok(store) = LocalStorage::get::<HistoryStore>(HISTORY_KEY) {
            return store;
        }
        HistoryStore {
            history: Vec::new(),
        }
    }

    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn push(&mut self, item: String) {
        self.history.push(item);
        let _ = LocalStorage::set(HISTORY_KEY, self);
    }

    pub fn clear(&mut self) {
        self.history = Vec::new();
        let _ = LocalStorage::set(HISTORY_KEY, self);
    }
}
