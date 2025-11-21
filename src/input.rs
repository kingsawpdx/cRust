use crossterm::event::{self, Event, KeyCode};
use crate::app::App;

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? 
        && let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => app.quit(),
                KeyCode::Char(' ') => app.increment(),
                _ => {}
            }
        
    }
    Ok(())
}
