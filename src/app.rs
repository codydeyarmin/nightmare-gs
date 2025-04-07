use std::error;

use ratatui::{buffer::Buffer, layout::Rect};

use gilrs::EventType as GamepadEventType;
use tokio::sync::mpsc;

use crate::{event::Event, pages::*, tasks::{ControllerTask, DriverEvent, DriverTask}};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub mode: Mode,
    pub counter: u32,
    pub display_x: Option<u16>,
    pub display_y: Option<u16>,
    pub page: Page,
    startup_page: StartupPage,
    controller_telem: ControllerTelem,
    control_panel: ControlPanel,
    controller_task: ControllerTask,
    driver_task: DriverTask,
    sender: Option<mpsc::UnboundedSender<Event>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: Mode::Running,
            counter: 0,
            display_x: None,
            display_y: None,
            page: Page::Startup,
            startup_page: StartupPage::new(),
            control_panel: ControlPanel::new(),
            controller_telem: ControllerTelem::new(),
            controller_task: ControllerTask::new(),
            driver_task: DriverTask::new(),
            sender: None,
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

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_sender(&mut self, sender: &mpsc::UnboundedSender<Event>) {
        self.sender = Some(sender.clone());
        self.driver_task.set_sender(sender.clone());
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

    pub fn control_panel_next_item(&mut self) {
        self.control_panel.next_item();
    }

    pub fn control_panel_previous_item(&mut self) {
        self.control_panel.previous_item();
    }

    pub fn control_panel_next_window(&mut self){
        self.control_panel.next_window();
    }
    pub fn control_panel_prev_window(&mut self){
        self.control_panel.prev_window();
    }

    pub fn control_panel_select(&mut self){
        if let Some(result) = self.control_panel.select(){
            match result {
                ControlResult::ChangePage(page) => self.change_page(page),
                ControlResult::SetController(controller) => {
                    self.controller_task.add_sender(self.sender.as_ref().unwrap());
                    self.controller_task.add_controller(controller);
                    self.controller_task.add_task();
                },
                ControlResult::DriverChange(event) => {
                    match event {
                        _ => ()
                    }
                }
            }
        }
    }

    pub fn render_current_page(&self, area: Rect, buf: &mut Buffer) {
        match self.page {
            Page::Startup => self.startup_page.render(area, buf),
            Page::ControllerTelem => self.controller_telem.render(area, buf),
        }
    }

    pub fn render_terminal_page(&self, area: Rect, buf: &mut Buffer) {
         self.control_panel.render(area, buf);
    }

    pub fn handle_controller_event(&mut self, event: GamepadEventType){
        self.controller_telem.add_telem(event);
    }

    fn change_page(&mut self, page: Page){
        self.page = page;
    }

    pub fn handle_driver_event(&mut self, event: DriverEvent) {
        
    }

}
