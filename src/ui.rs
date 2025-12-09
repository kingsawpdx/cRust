use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize, Color},
    widgets::{Block, Borders, Paragraph, List, ListItem, Wrap},
    text::{Line},
    Frame,
};
use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &mut App) {

    let outer_layout = Layout::default()
         .direction(Direction::Horizontal)
         .constraints(vec![
             Constraint::Percentage(30),
             Constraint::Percentage(40),
             Constraint::Percentage(30)
         ])
         .split(frame.area());

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(outer_layout[0]);

    let bottom_text = vec![
        Line::from("1) Press the spacebar to make a sandwich."),
        Line::from("2) Use the up and down arrow keys to navigate the store."),
        Line::from("3) Once you have enough sandwiches, press (p) to purchase the selected item in the store."),
        Line::from("4) The game is over when your total sandwiches is >= 65,535."),
        Line::from("5) Press (q) to quit."),
    ];

    let store_items = vec![
        ("Sandwich Artist", app.current_player.get_upgrade_cost(0)),
        ("cRust-way", app.current_player.get_upgrade_cost(1)),
        ("cRust-azon", app.current_player.get_upgrade_cost(2)),
    ];

    let info_items = vec![
        ("Available Sandwiches", app.current_player.available_sandwiches),
        ("Sandwiches per Second", app.current_player.sandwiches_per_second),
        ("Total Sandwiches Made", app.current_player.total_sandwiches_made),
    ];

    let automation_info = vec![
        (
            "Sandwich Artist",
            app.current_player.sandwich_artists,
            1,
            "🥪"
        ),
        (
            "cRust-way",
            app.current_player.crustway,
            5,
            "🏪"
        ),
        (
            "cRust-azon",
            app.current_player.crustazon,
            10,
            "📦"
        ),
    ];

    let mut automation_text: Vec<Line> = Vec::new();
    automation_text.push(Line::from(""));

    for (name, owned, per_item, symbol) in automation_info {
        let total_production = owned * per_item;
        
        automation_text.push(Line::from(format!("{}", name)).bold());
        automation_text.push(Line::from(format!("  Owned: {}", owned)));
        automation_text.push(Line::from(format!("  Each generates: {} sandwich{}/sec", 
            per_item, 
            if per_item != 1 { "es" } else { "" }
        )));
        automation_text.push(Line::from(format!("  Total output: {} sandwich{}/sec", 
            total_production,
            if total_production != 1 { "es" } else { "" }
        )));

    if owned > 0 {
            let display_count = owned.min(20);
            let symbols = symbol.repeat(display_count as usize);
            let overflow = if owned > 20 { 
                format!(" +{} more", owned - 20) 
            } else { 
                String::new() 
            };
            automation_text.push(Line::from(format!("  {}{}", symbols, overflow)));
        }
        
        automation_text.push(Line::from(""));
    }

    let row_width = 30;

    let items: Vec<ListItem> = store_items
    .iter()
    .map(|(name, value)| {
        let line = dotted(name, &value.to_string(), row_width);
        ListItem::new(line)
    })
    .collect();

    let mut info_text: Vec<Line> = info_items
    .iter()
    .map(|(name, value)| {
        let line = dotted(name, &value.to_string(), 40);
            Line::from(line)
    })
    .collect();

    let progress_percent = (app.current_player.total_sandwiches_made as f64 / u16::MAX as f64) * 100.0;
    info_text.push(Line::from(format!("")));
    info_text.push(Line::from(format!("Victory Progress: {:.2}%", progress_percent)));

    info_text.insert(0, Line::from(format!("")));
    info_text.insert(0, Line::from(format!("Employee of the month: {}", app.current_player.name)));
    info_text.insert(4, Line::from(format!("")));

    let list = List::new(items)
        .block(Block::bordered().title("Store"))
        .style(Style::new().white())
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black)
                .bold()
        )
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_widget(
        Paragraph::new(info_text)
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
            )
            .wrap(Wrap { trim: true }),
        left_layout[1]);

    frame.render_widget(
        Paragraph::new(automation_text)
            .block(Block::new()
                   .borders(Borders::ALL)
                   .title("Automation")
            )
            .wrap(Wrap {trim: true}),
        outer_layout[1]);

    frame.render_stateful_widget(list, outer_layout[2], &mut app.list_state);

    fn dotted(label: &str, value: &str, width: usize) -> String {
        let dots = ".".repeat(width.saturating_sub(label.len() + value.len()));
        format!("{label}{dots}{value}")
    }

}
