use crate::player::Player;

pub struct App {
    pub running: bool,
    pub current_player: Player,
}

impl App {
    pub fn new(new_player: Player) -> Self {
        Self {
            running: true,
            current_player: new_player
        }
    }

    pub fn increment(&mut self) {
	self.current_player.increment_sandwiches();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
