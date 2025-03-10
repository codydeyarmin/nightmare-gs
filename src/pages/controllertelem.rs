use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Paragraph, Widget,},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ControllerTelem {
    row_index: usize,
}

impl ControllerTelem {
    pub fn new() -> Self {
        Self { row_index: 0 }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("This is the controller Telem page"))
            .block(
                Block::bordered()
                    .title("Controller Telem")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .centered()
            .render(area, buf);
    }
}