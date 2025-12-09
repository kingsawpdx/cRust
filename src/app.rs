use crate::player::Player;
use ratatui::widgets::ListState;

use std::time::Instant;

pub struct App {
    pub running: bool,
    pub current_player: Player,

    pub list_state: ListState,
    pub last_auto_increment: Instant,

    pub victory: bool,
}

impl App {
    pub fn new(new_player: Player) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            running: true,
            current_player: new_player,

            list_state: state,
            last_auto_increment: Instant::now(),

            victory: false,
        }
    }

    pub fn increment(&mut self) {
        self.current_player.increment_sandwiches();
        self.check_victory();
    }

    pub fn auto_increment(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_auto_increment);

        if elapsed.as_secs() >= 1 {
            let sandwiches_to_add = self.current_player.calculate_sandwiches_per_second();
            self.current_player.add_sandwiches(sandwiches_to_add);
            self.last_auto_increment = now;
            self.check_victory();
        }
    }

    fn check_victory(&mut self) {
        if self.current_player.has_won() {
            self.victory = true;
            self.running = false;
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_selection_down(&mut self, max: usize) {
        let i = self.list_state.selected().unwrap_or(0);
        if i + 1 < max {
            self.list_state.select(Some(i + 1));
        }
    }

    pub fn move_selection_up(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(i.saturating_sub(1)));
    }

    pub fn purchase(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        self.current_player.verify_funds(i as u16);
    }
}
