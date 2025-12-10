/// Player structure used to manage player data.
pub struct Player {

    /// Used to store player name.
    pub name: String,
    /// Used to store company name.
    pub company_name: String,

    /// Used to store number of sandwiches made.
    pub total_sandwiches_made: u16,
    /// Used to store number of sandwiches you can currently spend.
    pub available_sandwiches: u16,
    /// Used to store number of sandwiches being generated per second.
    pub sandwiches_per_second: u16,

    /// Used to store number of purchased sandwich artists.
    pub sandwich_artists: u16,
    /// Used to store number of purchased crustways.
    pub crustway: u16,
    /// Used to store number of purchased crustazons.
    pub crustazon: u16,
}

impl Player {

    /// Used to create a new player. Intakes a player name and company name.
    pub fn new(name: String, company_name: String) -> Self {
        Self {
            name,
            company_name,

            total_sandwiches_made: 0,
            available_sandwiches: 0,
            sandwiches_per_second: 0,

            sandwich_artists: 0,
            crustway: 0,
            crustazon: 0,
        }
    }

    /// Used to increment available sandwiches and total sandwiches made for the player.
    pub fn increment_sandwiches(&mut self) {
        self.total_sandwiches_made += 1;
        self.available_sandwiches += 1;
    }

    /// Used to modify total sandwiches and available sandwiches for every sandwich being generated a second.
    pub fn add_sandwiches(&mut self, amount: u16) {
        self.total_sandwiches_made = self.total_sandwiches_made.saturating_add(amount);
        self.available_sandwiches = self.available_sandwiches.saturating_add(amount);
    }

    /// Used to determine how many sandwiches are being generated every second.
    pub fn calculate_sandwiches_per_second(&mut self) -> u16 {
        let sps = self.sandwich_artists + self.crustway * 5 + self.crustazon * 10;

        self.sandwiches_per_second = sps;

        sps
    }

    /// Used to determine cost of store item depending on quantity owned.
    pub fn get_upgrade_cost(&self, item: u16) -> u16 {
        let (base_cost, owned) = match item {
            0 => (10, self.sandwich_artists),
            1 => (200, self.crustway),
            2 => (1000, self.crustazon),
            _ => return 0,
        };
        let multiplier = 1.15_f64.powi(owned as i32);
        (base_cost as f64 * multiplier).ceil() as u16
    }

    /// Used to verify user has necessary sandwiches available to be spent.
    pub fn verify_funds(&mut self, item: u16) -> bool {
        let price = self.get_upgrade_cost(item);

        if self.available_sandwiches >= price {
            self.available_sandwiches -= price;
            match item {
                0 => self.sandwich_artists += 1,
                1 => self.crustway += 1,
                2 => self.crustazon += 1,
                _ => {}
            }
            true
        } else {
            false
        }
    }

    /// Used to check if user has reached win condition.
    pub fn has_won(&self) -> bool {
        self.total_sandwiches_made == u16::MAX
    }
}
