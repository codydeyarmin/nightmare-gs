use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Paragraph, Widget,},
};

use crate::tasks::DriverEvent;

#[derive(Debug, Clone,  Default, PartialEq, Eq)]
pub struct DriverTelem {
    row_index: usize,
    message: Option<String>,
}

impl DriverTelem {
    pub fn new() -> Self {
        Self { row_index: 0 , message: None}
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let string: String;
        if let Some(message) = self.message.as_ref() {
            string = format!("This is the Driver Telem page\n\nThe telem message is: \n {}", message);
        } else {
            string = format!("This is the driver Telem page")
        }

        Paragraph::new(string)
            .block(
                Block::bordered()
                    .title("Driver Telem")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .centered()
            .render(area, buf);
    }

    pub fn add_telem(&mut self, event: DriverEvent){
        let mut message: Option<String> = self.message.clone();
        match event {
            DriverEvent::StateReport(state) => message = Some(state.to_string()),
            _ => (),
        }
        self.message = message;
    }
}
