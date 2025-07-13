use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StoredStats {
    pub total_sessions: u32,
    pub total_chars: u64,
    pub total_mistakes: u64,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
}

