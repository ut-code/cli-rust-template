/// Application mode
#[derive(Default, PartialEq)]
pub enum Mode {
    #[default]
    Normal,
    Editing,
}

/// Application state for TUI
pub struct App {
    /// Whether the app should quit
    pub should_quit: bool,
    /// Current mode
    pub mode: Mode,
    /// Counter for demo purposes
    pub counter: i64,
    /// Current list selection index
    pub list_state_index: usize,
    /// Demo list items
    pub items: Vec<String>,
    /// Log messages
    pub logs: Vec<String>,
    /// Edit buffer
    pub edit_buffer: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self {
            should_quit: false,
            mode: Mode::Normal,
            counter: 0,
            list_state_index: 0,
            items: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ],
            logs: vec!["[INFO] Press ? for help".to_string()],
            edit_buffer: String::new(),
        }
    }

    /// Handle tick events (called periodically)
    pub fn tick(&mut self) {
        // Perform periodic updates here
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Increment counter
    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
        self.add_log(format!("Counter: {}", self.counter));
    }

    /// Decrement counter
    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
        self.add_log(format!("Counter: {}", self.counter));
    }

    /// Move list selection down
    pub fn list_next(&mut self) {
        if !self.items.is_empty() {
            self.list_state_index = (self.list_state_index + 1) % self.items.len();
        }
    }

    /// Move list selection up
    pub fn list_previous(&mut self) {
        if !self.items.is_empty() {
            self.list_state_index = if self.list_state_index == 0 {
                self.items.len() - 1
            } else {
                self.list_state_index - 1
            };
        }
    }

    /// Delete selected item
    pub fn delete_selected(&mut self) {
        if !self.items.is_empty() {
            let removed = self.items.remove(self.list_state_index);
            self.add_log(format!("Deleted: {}", removed));
            if self.list_state_index >= self.items.len() && self.list_state_index > 0 {
                self.list_state_index -= 1;
            }
        }
    }

    /// Add a new item
    pub fn add_item(&mut self, item: String) {
        self.items.push(item.clone());
        self.add_log(format!("Added: {}", item));
    }

    /// Start editing the selected item
    pub fn start_edit(&mut self) {
        if !self.items.is_empty() {
            self.edit_buffer = self.items[self.list_state_index].clone();
            self.mode = Mode::Editing;
        }
    }

    /// Confirm edit and save
    pub fn confirm_edit(&mut self) {
        if !self.edit_buffer.is_empty() {
            self.items[self.list_state_index] = self.edit_buffer.clone();
            self.add_log(format!("Updated: {}", self.edit_buffer));
        }
        self.edit_buffer.clear();
        self.mode = Mode::Normal;
    }

    /// Cancel edit
    pub fn cancel_edit(&mut self) {
        self.edit_buffer.clear();
        self.mode = Mode::Normal;
    }

    /// Add character to edit buffer
    pub fn edit_push(&mut self, c: char) {
        self.edit_buffer.push(c);
    }

    /// Remove last character from edit buffer
    pub fn edit_pop(&mut self) {
        self.edit_buffer.pop();
    }

    /// Add a log message
    pub fn add_log(&mut self, message: String) {
        self.logs.push(message);
        // Keep only last 100 logs
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }
}
