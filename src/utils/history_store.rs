use yew::format::Json;
use yew::services::storage::{StorageService, Area};

const HISTORY_KEY: &str = "portolio.history";

#[derive(Debug, Clone)]
pub struct HistoryStore {
    history: Vec<String>,
}

impl HistoryStore {
    pub fn new() -> Self {
        if let Ok(store) = StorageService::new(Area::Local) {
            if let Json(Ok(history)) = store.restore(HISTORY_KEY) {
                return HistoryStore { history }
            }
        }
        HistoryStore { history: Vec::new() }
    }

    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn push(&mut self, item: String) {
        self.history.push(item);
        // flush result if failed to get StorageResult
        let _ = StorageService::new(Area::Local).map(|mut s| {
            s.store(HISTORY_KEY, Json(&self.history));
        });
    }

    pub fn clear(&mut self) {
        self.history = Vec::new();
        // flush result if failed to get StorageResult
        let _ = StorageService::new(Area::Local).map(|mut s| {
            s.store(HISTORY_KEY, Json(&self.history));
        });
    }
}