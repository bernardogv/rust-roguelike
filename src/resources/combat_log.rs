/// Combat log for tracking combat messages

use bevy::prelude::*;
use std::collections::VecDeque;

// ============================================================================
// COMBAT LOG RESOURCE
// ============================================================================

/// Stores combat messages for player feedback
#[derive(Resource)]
pub struct CombatLog {
    messages: VecDeque<String>,
    max_messages: usize,
}

impl Default for CombatLog {
    fn default() -> Self {
        Self::new(10)
    }
}

impl CombatLog {
    pub fn new(max_messages: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(max_messages),
            max_messages,
        }
    }

    /// Add a message to the log
    pub fn add_message(&mut self, message: String) {
        info!("COMBAT LOG: {}", message);

        // Add message
        self.messages.push_back(message);

        // Remove oldest if over capacity
        while self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }

    /// Get all messages (most recent last)
    pub fn get_messages(&self) -> Vec<&String> {
        self.messages.iter().collect()
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
