use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Paragraph, Widget,},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StartupPage {
    row_index: usize,
}

impl StartupPage {
    pub fn new() -> Self {
        Self { row_index: 0 }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("This is the test startup page text"))
            .block(
                Block::bordered()
                    .title("StartupPage")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .centered()
            .render(area, buf);
    }
}
