# Name: Sawyer King

## Project Description:
cRust is an idle game that will be developed utilizing Rust. The objective of this assignment is to get hands on experience applying advanced concepts of the Rust programming language through the implementation of an interesting project. Due to the time constraint, cRust will be very similar to other modern idle games such as cookie clicker. 

## Project Vision:

### cRust will have the following capabilities:

A player class which manages all data for the user including:
- User name.
- Restaurant name.
- Current sandwiches available.
- Total sandwiches made.
- Sandwiches being generated per second.

A market that will allow for upgrades and automation. These upgrades will include:
- Sandwich artists
- cRust-Way sandwich shops
- cRust-azon distribution centers

The project will have a victory condition:
- When the players total sandwiches made reaches 65,535 sandwiches the game will end.

## Lessons Learned:

Ratatui: Ratatui was fun to work with but provided a few challenges. 
- Getting used to the general stucture of ratatui took me a little bit but now is very intuitive.
- Managing state with the list component was very similar experience to adapting to ratatui in general. At first it provided some challenges but after seeing examples in the documentation, it now feels second nature. 

Rust: This was a great way to gain some confidence in rust!
Generally there weren't many obstacles but here are some interesting things I encountered.
- This was the first application where I ran into size limitations. I originally used u8's for most values but had to change to u16 when I ran into crashing due to hitting the maximum value.
- Implementing the auto incrementer was a little tough to wrap my head around at first but turned out very well.
