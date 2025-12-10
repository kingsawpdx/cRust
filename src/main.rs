mod app;
mod input;
mod player;
mod ui;
use crate::player::Player;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io;

use app::App;
use input::handle_input;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ui::draw_ui;

/// Primary function running application.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nWelcome to cRust! A sandwich making idle game.");
    println!("   *Note: This game works best when played in full screen.\n");
    println!("To begin, please enter your name:");

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    println!("Now enter the name of your restaurant:");
    let mut company = String::new();
    io::stdin().read_line(&mut company)?;
    let company = company.trim().to_string();

    let new_player = Player::new(name, company);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(new_player);

    while app.running {
        terminal.draw(|f| draw_ui(f, &mut app))?;
        handle_input(&mut app)?;
        app.auto_increment();
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if app.victory {
        println!("\n *************** CONGRATULATIONS! ***************");
        println!("You've reached the maximum sandwich production!");
        println!(
            "Total Sandwiches Made: {}",
            app.current_player.total_sandwiches_made
        );
        println!(
            "\n{} and {} have conquered the sandwich industry!",
            app.current_player.name, app.current_player.company_name
        );
        println!("\nThank you for playing cRust!");
    } else {
        println!("\nThanks for playing!");
    }

    Ok(())
}
