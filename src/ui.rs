use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize, Color},
    widgets::{Block, Borders, Paragraph, List, ListItem, Wrap},
    text::{Line},
    Frame,
};
use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &mut App) {

    // ----------------------- Layouts --------------------------------

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

    // ----------------------- Left --------------------------------
    
    let bottom_text = vec![
        Line::from("1) Press the spacebar to make a sandwich."),
        Line::from("2) Press (q) to quit."),
    ];


    // ----------------------- Right --------------------------------


    let store_items = vec![
        ("Sandwich Artist", "10"),
        ("cRust-way", "20"),
        ("cRust-azon", "30"),
    ];

    let info_items = vec![
        ("Available Sandwiches", app.current_player.available_sandwiches),
        ("Total Sandwiches Made", app.current_player.total_sandwiches_made),
        ("Sandwiches per Second", app.current_player.sandwiches_per_second),
    ];

    let row_width = 30;

    let items: Vec<ListItem> = store_items
    .iter()
    .map(|(name, value)| {
        let line = dotted(name, value, row_width);
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


    info_text.insert(0, Line::from(format!("")));
    info_text.insert(0, Line::from(format!("Employee of the month: {}", app.current_player.name)));

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
        //Paragraph::new(text)
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
        Paragraph::new("Middle")
            .block(Block::new()
                   .borders(Borders::ALL)
                   .title("Automation")
            ),
        outer_layout[1]);

    frame.render_stateful_widget(list, outer_layout[2], &mut app.list_state);

    fn dotted(label: &str, value: &str, width: usize) -> String {
        let dots = ".".repeat(width.saturating_sub(label.len() + value.len()));
        format!("{label}{dots}{value}")
    }

}
