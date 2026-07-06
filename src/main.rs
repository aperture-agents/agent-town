//! /src/main.rs
//!
//! Main Agent Town Game Loop
//!
//! Controls the Game flow of Agent Town
//!

mod game;

use crate::game::state::{GamePhase, GameState};

fn main() {
    // Create the game
    let mut game_state = GameState::new();

    // Run Game Loop
    loop {
        match game_state.phase {
            GamePhase::Start => game_state.start_game(),
            GamePhase::Discussion => game_state.start_discussion(),
            GamePhase::Voting => game_state.start_voting(),
            GamePhase::Night => game_state.start_night(),
            GamePhase::GameOver => game_state.game_over(),
        }
    }
}
