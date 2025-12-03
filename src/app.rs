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

    /*pub fn purchase(&mut self){
        let i = self.list_state.selected().unwrap_or(0);
        
        match i {
            0 => self.current_player.verify_funds(10, i.try_into().unwrap()),
            1 => self.current_player.verify_funds(100, i.try_into().unwrap()),
            2 => self.current_player.verify_funds(200, i.try_into().unwrap()),
            _ => { }

        }

    }*/

    pub fn purchase(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);

        let price: u16 = match i {
            0 => 10,
            1 => 100,
            2 => 200,
            _ => return,
        };

        self.current_player.verify_funds(price, i as u16);
    }

}
