use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::*;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    let vertical = Layout::vertical([
        Constraint::Percentage(65),
        Constraint::Percentage(35),
    ]);
    let [page, control_panel] = vertical.areas(area);
    app.render_terminal_page(control_panel, frame.buffer_mut());
    app.render_current_page(page, frame.buffer_mut());
}
