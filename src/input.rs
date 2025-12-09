use crate::app::App;
use crossterm::event::{self, Event, KeyCode};

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))?
        && let Event::Key(key) = event::read()?
    {
        match key.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char(' ') => app.increment(),
            KeyCode::Char('p') => app.purchase(),

            KeyCode::Up => app.move_selection_up(),
            KeyCode::Down => app.move_selection_down(3),

            _ => {}
        }
    }
    Ok(())
}
