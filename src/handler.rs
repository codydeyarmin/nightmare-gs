use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            app.control_panel_next_window();
        }
        KeyCode::Left => {
            app.control_panel_prev_window();
        }
        // Other handlers you could add here.
        KeyCode::Down => {
            app.control_panel_next_item();
        }

        KeyCode::Up => {
            app.control_panel_previous_item();
        }

        KeyCode::Enter => {
            app.control_panel_select();
        }
        _ => {}
    }
    Ok(())
}
