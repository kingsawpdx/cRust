use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    text::{Line},
    Frame,
};
use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(frame.area());

    let block = Block::default()
        .title(app.current_player.company_name.clone())
        .borders(Borders::ALL);

    let text = vec![
        Line::from(format!("Available Sandwiches: {}", app.current_player.available_sandwiches)),
        Line::from(format!("Total Sandwiches Made: {}", app.current_player.total_sandwiches_made)),
        Line::from(format!("Sandwiches per Second: {}", app.current_player.sandwiches_per_second)),
    ];

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, layout[0]);

    let bottom_block = Block::default()
        .title("How to play:")
        .borders(Borders::ALL);

    let bottom_text = vec![
        Line::from(format!("1) Press the spacebar to make a sandwich.")),
        Line::from(format!("2) Press (q) to quit.")),
    ];

    let bottom_paragraph = Paragraph::new(bottom_text).block(bottom_block);
    frame.render_widget(bottom_paragraph, layout[1]);
        
}
