mod player;
use crate::player::Player;

fn main() {
    let mut instance = Player ::new("Sawyer".to_string(), "test".to_string());

    instance.display_data();
    instance.increment_sandwiches();
    instance.display_data();
}
