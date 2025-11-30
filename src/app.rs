use crate::player::Player;

use ratatui::widgets::ListState;

pub struct App {
    pub running: bool,
    pub current_player: Player,

    pub list_state:ListState,
}

impl App {
    pub fn new(new_player: Player) -> Self {

        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            running: true,
            current_player: new_player,

            list_state: state,
        }
    }

    pub fn increment(&mut self) {
	self.current_player.increment_sandwiches();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_selection_up(&mut self, max: usize) {
        let i = self.list_state.selected().unwrap_or(0);
        if i + 1 < max {
            self.list_state.select(Some(i + 1));
        }
    }

    pub fn move_selection_down(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(i.saturating_sub(1)));
    }

}
