use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::*;

pub fn control_panel_size(app: &App) -> u16 {
    match app.get_y() {
        Some(y) => {
            if y > MIN_CONTROL_PANEL_HEIGHT {
                y
            } else {
                MIN_CONTROL_PANEL_HEIGHT
            }
        }
        None => MIN_CONTROL_PANEL_HEIGHT,
    }
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    let vertical = Layout::vertical([
        Constraint::Min(15),
        Constraint::Length(MIN_CONTROL_PANEL_HEIGHT),
    ]);
    let [page, control_panel] = vertical.areas(area);
    // frame.render_widget(
    //     Paragraph::new(format!(
    //         "This is a tui template.\n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //         app.counter
    //     ))
    //     .block(
    //         Block::bordered()
    //             .title("Template")
    //             .title_alignment(Alignment::Center)
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    //     .centered(),
    //     control_panel,
    // );
    app.render_terminal_page(control_panel, frame.buffer_mut());
    app.render_current_page(page, frame.buffer_mut());
}
