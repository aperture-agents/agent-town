//! /src/main.rs
//!
//! Main Agent Town Game Loop
//!
//! Controls the Game flow of Agent Town
//!

mod game;

use crate::game::state::GameState;

fn main() {
    // Create the game
    let mut game_state = GameState::new();

    // Start the game
    game_state.start_game();

    // Run Game Loop - maybe use match instead?
    loop {
        // Hold Discussion Period

        // Start Voting Phase

        // Start Night Phase

        // Perform Actions - Check Win Condition

        // End Game or Begin Discussion Phase

        // Recap Round Events (who died etc)
    }
}
