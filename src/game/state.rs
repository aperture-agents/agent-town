//! /src/game/state.rs
//!
//! State module.
//!
//! Contains structs and logic related to maintaining the game state.
//!

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::io;

use crate::game::chat::Chat;
use crate::game::player::{Action, Player, Role};

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
    Start,
    Discussion,
    Voting,
    Night,
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
    #[allow(dead_code)] // TODO: IMPL game history with chat
    pub prev_rounds: Vec<Round>,
    pub curr_round: Round,
    pub num_mafia: usize,
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

        // Prompt for the player names
        let players = prompt_players(num_mafia);

        Self {
            phase: GamePhase::Start,
            players: players,
            prev_rounds: Vec::new(),
            curr_round: Round::new(),
            num_mafia: num_mafia,
        }
    }

    /// start_game()
    ///
    /// Start of the game activities.
    ///     - Assign Roles
    ///     - Show Players their roles.
    ///     - Explain the rules of the game.
    ///     - start Discussion Phase.
    ///
    pub fn start_game(&mut self) {
        // Assign all players roles
        self.assign_roles();

        // TODO: Should show players their roles but not necessary for this first demo version
        // TODO: Same with explaining rules and such
        // TODO: Maybe introduce some temp function to print to a player unique file which will
        // TODO: Eventually become the interface for the LLMs. For now we can ignore this I think.
        // for player in self.players {
        //      player.role_ping();
        //      player.rule_ping();
        //  }

        // start the first discussion phase
        self.phase = GamePhase::Discussion;
    }

    /// assign_roles()
    ///
    /// Helper function to randomly assign roles to our Players
    /// One Person will be Doctor.
    /// One Person will be Detective.
    /// Create num_mafia Mafia Members.
    /// The rest are assgned Villager.
    ///
    /// TODO: Maybe allow all roles to be dynamic in counts.
    ///
    fn assign_roles(&mut self) {
        // Assign all players to unassigned - explicit
        for player in &mut self.players {
            player.role = Role::Unassigned
        }

        // Shuffle the players
        let mut rng = thread_rng();
        self.players.shuffle(&mut rng);

        // Assign the first num_mafia as Mafia
        self.players[0..self.num_mafia]
            .iter_mut()
            .for_each(|player| player.role = Role::Mafia);

        // Assign the next as Doctor
        self.players[self.num_mafia].role = Role::Doctor;

        // Assign the next as Detective
        self.players[self.num_mafia + 1].role = Role::Detective;

        // Assign the rest as Villager
        self.players[self.num_mafia + 2..].iter_mut().for_each(|player| player.role = Role::Villager);
    }

    /// start_voting()
    ///
    /// Trigger voting period of the Round.
    ///
    /// Allow players to cast votes and log them.
    /// Optionally eliminate a Player if voted majority.
    /// TODO: Maybe record the votes in the round information?
    ///
    fn start_voting(&mut self) {
        // Allow each player to vote
        let mut votes = Vec::new();
        for player in &self.players {
            votes.push(player.vote(&self.players));
        }

        // Tally the votes in a map
        let mut map = HashMap::new();
        for vote in &votes {
            if let Some(player) = vote {
                *map.entry(player.id).or_insert(0) += 1;
            }
        }

        // Announce a death if there is one and remove the player
        let death = map.iter().max_by_key(|(_, v)| *v);
        if let Some((id, votes)) = death {
            let dead_player = self.players.iter().find(|p| p.id == *id).expect("Failed to find dead player - BUG");
            println!("Player {} received {} votes, and is eliminated.", dead_player.name, votes);
            dead_player.kill();

        } else {
            println!("No one was voted out...");
        }

        // Check if the game should end - maybe mafia won
        self.is_game_over();

        // Set the phase to Night Time
        self.phase = GamePhase::Night;
    }

    /// is_game_over()
    ///
    /// Helper function to check if the game should end.
    ///
    /// The game should end if:
    ///     - All Mafia voted out.
    ///     - Mafia holds parity with non Mafia count.
    ///
    fn is_game_over(&mut self) {}

    /// start_night()
    ///
    /// Start the Night Phase.
    /// Allows special jobs to act().
    ///
    fn start_night() {}
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
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to Read Line");
        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("Please enter a valid number");
                input.clear();
                continue;
            }
        };
    }
}

/// prompt_players()
///
// Prompt to create our players
// Minimum # of Villagers to ensure Mafia CANNOT have Majority at Start
// Minimum # of Players to ensure Num Mafia can be fullfilled, 2 special roles,
// And # of Villagers is enough to ensure Mafia CANNOT have Majority at Start.
//
// Args:
//  num_mafia: The number of player selected Mafia roles
//
// Returns:
//  List of Players that fits conditions
//
fn prompt_players(num_mafia: usize) -> Vec<Player> {
    let mut names = Vec::new();
    let min_villagers: usize = num_mafia - 2;
    let min_players: usize = min_villagers + 2 + num_mafia;
    let mut uuid: usize = 0;
    let mut input = String::new();
    loop {
        input.clear();
        println!(
            "Enter the name of player #{} or Enter to Stop",
            names.len() + 1
        );
        io::stdin().read_line(&mut input).expect("Failed to Read Line");
        match input.trim() {
            "" => {
                // Determine if the current list satisfies acceptable player count
                if names.len() < min_players {
                    println!(
                        "You currently do not fulfill the acceptable minimal player count of: {}, add more players",
                        min_players
                    );
                    continue;
                } else {
                    break;
                }
            }
            name => {
                names.push(Player::new(uuid, String::from(name), Role::Unassigned));
                uuid += 1;
            }
        }
    }
    names
}

/// Round
///
/// A Round contains information for each given game round.
///
/// Args:
///     chat:   The chat instance for this round.
///     acts:   The record of actions for this round.
///
pub struct Round {
    chat: Chat,
    acts: Vec<Action>,
}

impl Round {
    pub fn new() -> Self {
        Self {
            chat: Chat::new(),
            acts: Vec::new(),
        }
    }
}
