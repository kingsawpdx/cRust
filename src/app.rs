use crate::player::Player;
use ratatui::widgets::ListState;

use std::time::Instant;

/// A structure that maintains the overall application status
pub struct App {
    /// Used to monitor game status.
    pub running: bool,

    /// Used to store information for the current player.
    pub current_player: Player,

    /// Used to keep track of what store item is currently selected.
    pub list_state: ListState,

    /// Used to manage sandwich generating per second.
    pub last_auto_increment: Instant,

    /// Used to determine win condition.
    pub victory: bool,
}

/// Main app implementation
impl App {

    /// Constructor for app. It intakes a player object to manage its data.
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

    /// Function used to handle sandwich making when space bar is pressed. 
    pub fn increment(&mut self) {
        self.current_player.increment_sandwiches();
        self.check_victory();
    }

    /// Function used to handle generating sandwiches every second.
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

    /// Function used to see if user has reached win condition.
    fn check_victory(&mut self) {
        if self.current_player.has_won() {
            self.victory = true;
            self.running = false;
        }
    }

    /// Function used to terminate game.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Function used to change store selection. Moves selection down.
    pub fn move_selection_down(&mut self, max: usize) {
        let i = self.list_state.selected().unwrap_or(0);
        if i + 1 < max {
            self.list_state.select(Some(i + 1));
        }
    }

    /// Function used to change store selection. Moves selection up.
    pub fn move_selection_up(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(i.saturating_sub(1)));
    }

    /// Function used to purchase selected item in store.
    pub fn purchase(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        self.current_player.verify_funds(i as u16);
    }
}
