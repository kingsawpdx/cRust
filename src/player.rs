pub struct Player {

   pub name: String,
   pub company_name: String,

   pub total_sandwiches_made: u8,
   pub available_sandwiches: u8,
   pub sandwiches_per_second: u8,

}

impl Player {

   pub fn new(name: String, company_name: String) -> Self {
      Self {
        name,
        company_name,
        
        total_sandwiches_made: 0,
        available_sandwiches: 0,
        sandwiches_per_second: 0
      }
   }

  /* pub fn display_data(&self) {
         println!("Name: {}", self.name);
         println!("Company name: {}", self.company_name);
         println!("Total sandwiches made: {}", self.total_sandwiches_made);
         println!("Sandwiches available: {}", self.available_sandwiches);
         println!("Sandwiches per second: {}", self.sandwiches_per_second);
   }*/

   pub fn increment_sandwiches(&mut self) {
      self.total_sandwiches_made += 1;
      self.available_sandwiches += 1;
   }
}
