//! /src/game/state.rs
//!
//! State module.
//!
//! Contains structs and logic related to maintaining the game state.
//!

use std::io::{self, Read};

use crate::game::player::{Player, Action};
use crate::game::chat::Chat;

/// GamePhase
///
/// An Enum for the various game phases.
///
/// Variants:
///     Discussion: Phase when players discuss what to do.
///     Voting: Short Phase when players vote out a player.
///     Night: Short Phase when unique roles chose their targets.
///     Start: Start of the game, rules and role assignemnts.
///
pub enum GamePhase {
    Discussion,
    Voting,
    Night,
    Start,
}

/// GameState
///
/// An instance responsible for maintaining the state of the game.
///
/// Args:
///     phase:      The Current GamePhase.
///     players:    A List of Players.
///     rounds:     A List of round history.
///
pub struct GameState {
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub prev_rounds: Vec<Round>,
    pub num_mafia: u32,
}

impl GameState {
    /// new()
    ///
    /// Creates a new instance of the GameState.
    ///
    /// Will Prompt the User for num_mafia.
    /// Will prompt the user for player names until:
    ///     - All roles assigned (1 detective, 1 doctor, num_mafia, >=1 villager)
    ///     - Mafia doesnt instantly win (#mafia == #!mafia).
    ///
    /// Args:
    ///     players: List of Players.
    ///
    /// Returns:
    ///     GameState - new GameState instance.
    ///
    pub fn new() -> Self {
        // Prompt for the number of Mafia
        let num_mafia = prompt_num_mafia();

        // Prompt to create our players
        // Minimum # of Villagers to ensure Mafia CANNOT have Majority at Start
        // Minimum # of Players to ensure Num Mafia can be fullfilled, 2 special roles,
        // And # of Villagers is enough to ensure Mafia CANNOT have Majority at Start.
        let mut names = Vec::new();
        let min_villagers: usize = num_mafia - 2;
        let min_players: usize = min_villagers + 2 + num_mafia;
        loop {
            println!("Enter the name of player #{} or Enter to Stop", names.len()+1);
            match input.trim() {
                "" => {
                    // Determine if the current list satisfies acceptable player count
                    if names.len() < min_players {
                        println!("You currently do not fulfill the acceptable minimal player count of: {}, add more players", min_players);
                        continue;
                    } else {
                        break;
                    }
                },
                name => names.push(String::from(name)),
            }
        }

        // Create players and assign roles
       let players = assign_roles();

        Self {
            phase: GamePhase::Start,
            players: players,
            prev_rounds: Vec::new(),
            num_mafia: num_mafia,
        }
    }

    /// start_game()
    ///
    /// Start of the game activities.
    ///     - Show Players their roles.
    ///     - Explain the rules of the game.
    ///     - Begin Discussion Phase.
    ///
    pub fn start_game(&mut self) {

    }
}

/// prompt_num_mafia()
///
/// Helper function to prompt the user for the number of mafia players to create.
///
/// Returns:
///     The number of requested mafia players.
///
fn prompt_num_mafia() -> usize {
    println!("Enter the number of Mafia");
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("Failed to Read Line");
        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("Please enter a valid number");
                input.clear();
                continue;
            }
        };
    };
}

/// prompt_player_names()
///



/// assign_roles()
///
/// Helper function to create Players and randomly assign them roles.
/// One Person will be Doctor.
/// One Person will be Detective.
/// Create num_mafia Mafia Members.
/// The rest are assgned Villager.
///
/// This function assumes there are enough names provided to achieve the above conditions.
///
/// Args:
///     names: The provided player names.
///     num_mafia: The number of mafia players to make.
///
/// Returns:
///     A list of created Players
///
fn assign_roles(names: Vec<String>, num_mafia: usize) -> Vec<Player> {}

/// Round
///
/// A Round contains information for each given game round.
///
/// Args:
///     chat:   The chat instance for this round.
///     acts:   The record of actions for this round.
///
struct Round {
    chat: Chat,
    acts: Vec<Action>,
}
