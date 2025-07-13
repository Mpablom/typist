use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypingStats {
    pub total_chars: usize,
    pub mistakes: usize,
    pub wpm: f64,
}
impl TypingStats {
    pub fn accuracy(&self) -> f64 {
        if self.total_chars == 0 {
            100.0
        } else {
            100.0 * (self.total_chars.saturating_sub(self.mistakes)) as f64
                / self.total_chars as f64
        }
    }

    #[allow(dead_code)]
    pub fn update_wpm(&mut self, seconds: f64) {
        if seconds > 0.0 {
            self.wpm = (self.total_chars as f64 / 5.0) / (seconds / 60.0);
        }
    }
}
