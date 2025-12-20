use crate::config::CMD_INPUT_HIST_SIZE;
use std::collections::VecDeque; // VecDeque seems better
use tui_input::Input;


pub struct InputHistory {
    pub input: Input,
    pub history: VecDeque<String>,
    pub history_index: Option<usize>,
    max_size: usize,
}

impl Default for InputHistory {
    fn default() -> Self {
        Self {
            input: Input::default(),
            history: VecDeque::new(),
            history_index: None,
            max_size: CMD_INPUT_HIST_SIZE,
        }
    }
}

impl InputHistory {
	
	pub fn set_max_size(max_size: usize) -> Self {
        Self {
            input: Input::default(),
            history: VecDeque::new(),
            history_index: None,
            max_size,
        }
    }
    
    pub fn push(&mut self, entry: String) {
        if entry.trim().is_empty() {
            return;
        }
        if self.history.contains(&entry) {
            return;
        }
        if self.history.len() >= self.max_size {
            self.history.pop_front();
        }
        self.history.push_back(entry);
        self.history_index = None;
    }

    pub fn up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let len = self.history.len();
        let new_index = match self.history_index {
            None => len - 1,
            Some(0) => 0,
            Some(i) => i - 1,
        };
        self.history_index = Some(new_index);
        self.input = Input::new(self.history[new_index].clone());
    }

    pub fn down(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let len = self.history.len();
        match self.history_index {
            None => {}
            Some(i) if i >= len - 1 => {
                self.history_index = None;
                self.input = Input::default();
            }
            Some(i) => {
                self.history_index = Some(i + 1);
                self.input = Input::new(self.history[i + 1].clone());
            }
        }
    }
}
