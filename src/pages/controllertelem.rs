use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Paragraph, Widget,},
};

use gilrs::EventType as GamepadEventType;

#[derive(Debug, Clone,  Default, PartialEq, Eq)]
pub struct ControllerTelem {
    row_index: usize,
    message: Option<String>,
}

impl ControllerTelem {
    pub fn new() -> Self {
        Self { row_index: 0 , message: None}
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let string: String;
        if let Some(message) = self.message.as_ref() {
            string = format!("This is the controller Telem page\n\nThe telem message is: \n {}", message);
        } else {
            string = format!("This is the controller Telem page")
        }

        Paragraph::new(string)
            .block(
                Block::bordered()
                    .title("Controller Telem")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .centered()
            .render(area, buf);
    }

    pub fn add_telem(&mut self, event: GamepadEventType){
        let mut message: Option<String> = self.message.clone();
        match event {
            GamepadEventType::AxisChanged(axis, val, _) => {
                message = Some(format!("Axis {:?} changed by {}", axis, val));
            },
            GamepadEventType::ButtonPressed(button, code) => {
                message = Some(format!("Button {:?} pressed with code {:?}", button, code));
            },
            GamepadEventType::ButtonReleased(..) => (),
            GamepadEventType::ButtonChanged(..) => (),
            GamepadEventType::ButtonRepeated(..) => (),
            GamepadEventType::Connected => {
                message = Some(format!("Controller connected",));
            },
            GamepadEventType::Disconnected => {
                message = Some(format!("Controller disconnected",));
            },
            _ => ()
        }
        self.message = message;
    }

}