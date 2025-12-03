pub struct Player {

   pub name: String,
   pub company_name: String,

   pub total_sandwiches_made: u16,
   pub available_sandwiches: u16,
   pub sandwiches_per_second: u16,

   pub sandwich_artists: u16,
   pub crustway: u16,
   pub crustazon: u16,

}

impl Player {

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

   pub fn increment_sandwiches(&mut self) {
      self.total_sandwiches_made += 1;
      self.available_sandwiches += 1;
   }

   pub fn verify_funds(&mut self, price: u16, item: u16){
      if self.available_sandwiches >= price {
         self.available_sandwiches = self.available_sandwiches - price;
         match item {
            0 => { self.sandwich_artists = self.sandwich_artists + 1 },
            1 => { self.crustway = self.crustway + 1 },
            2 => { self.crustazon = self.crustazon + 1 },
            _ => { }
         }
      }
   }

}
