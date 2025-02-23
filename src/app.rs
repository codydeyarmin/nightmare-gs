use std::error;

use ratatui::{buffer::Buffer, layout::Rect};

use stick::Controller;

use crate::pages::*;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub const MIN_CONTROL_PANEL_HEIGHT: u16 = 15;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub mode: Mode,
    pub counter: u32,
    pub display_x: Option<u16>,
    pub display_y: Option<u16>,
    pub page: Pages,
    startup_page: StartupPage,
    terminal: Terminal,
    pub controller: Option<stick::Controller>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: Mode::Running,
            counter: 0,
            display_x: None,
            display_y: None,
            page: Pages::Startup,
            startup_page: StartupPage::new(),
            terminal: Terminal::new(),
            controller: None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Running,
    Destroy,
    Quit,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Pages {
    #[default]
    Startup,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.mode = Mode::Quit;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn set_x_y(&mut self, x: u16, y: u16) {
        self.display_x = Some(x);
        self.display_y = Some(y);
    }

    pub fn get_x(&self) -> Option<u16> {
        self.display_x
    }

    pub fn get_y(&self) -> Option<u16> {
        self.display_y
    }

    pub fn render_current_page(&self, area: Rect, buf: &mut Buffer) {
        match self.page {
            Pages::Startup => self.startup_page.render(area, buf),
        }
    }

    pub fn render_terminal_page(&self, area: Rect, buf: &mut Buffer) {
         self.terminal.render(area, buf);
    }


}
