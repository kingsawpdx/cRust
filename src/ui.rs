use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    text::{Line},
    Frame,
};
use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &App) {

    let outer_layout = Layout::default()
         .direction(Direction::Horizontal)
         //.margin(1)
         .constraints(vec![
             Constraint::Percentage(30),
             Constraint::Percentage(40),
             Constraint::Percentage(30)
         ])
         .split(frame.area());

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(outer_layout[0]);

    // ----------------------- Left Top --------------------------------
    
    let text = vec![
        Line::from(format!("Employee of the month: {}", app.current_player.name)),
        Line::from(format!("Available Sandwiches: {}", app.current_player.available_sandwiches)),
        Line::from(format!("Total Sandwiches Made: {}", app.current_player.total_sandwiches_made)),
        Line::from(format!("Sandwiches per Second: {}", app.current_player.sandwiches_per_second)),
    ];
    
    let bottom_text = vec![
        Line::from("1) Press the spacebar to make a sandwich."),
        Line::from("2) Press (q) to quit."),
    ];

    frame.render_widget(
        Paragraph::new(text)
            .block(Block::new()
                   .borders(Borders::ALL)
                   .title(app.current_player.company_name.clone())
            ),
        left_layout[0]);

    frame.render_widget(
        Paragraph::new(bottom_text)
            .block(Block::new()
                   .borders(Borders::ALL)
                   .title("How to play:")
            ),
        left_layout[1]);

    frame.render_widget(
        Paragraph::new("Middle")
            .block(Block::new()
                   .borders(Borders::ALL)
                   .title("Automation")
            ),
        outer_layout[1]);

    frame.render_widget(
        Paragraph::new("Right")
            .block(Block::new()
                   .borders(Borders::ALL)
                   .title("Store")
            ),
        outer_layout[2]);
/*
    let block = Block::default()
        .title(app.current_player.company_name.clone())
        .borders(Borders::ALL);

    let text = vec![
        Line::from(format!("Available Sandwiches: {}", app.current_player.available_sandwiches)),
        Line::from(format!("Total Sandwiches Made: {}", app.current_player.total_sandwiches_made)),
        Line::from(format!("Sandwiches per Second: {}", app.current_player.sandwiches_per_second)),
    ];

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, left_layout[0]);

    // ----------------------- Left Bottom --------------------------------

    let bottom_block = Block::default()
        .title("How to play:")
        .borders(Borders::ALL);

    let bottom_text = vec![
        Line::from(format!("1) Press the spacebar to make a sandwich.")),
        Line::from(format!("2) Press (q) to quit.")),
    ];

    let bottom_paragraph = Paragraph::new(bottom_text).block(bottom_block);
    frame.render_widget(bottom_paragraph, left_layout[1]);
  */      
}
