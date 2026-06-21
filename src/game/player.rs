//! /src/game/player.rs
//!
//! Player module
//!
//! Contains structs and logic related to game players.
//!

use std::io;

/// Role
///
/// The various Player Roles in Mafia, Roles dictate a Player's actions in the game
///
/// Roles:
///     Villager - They are 'innocent', works to find Mafia members and vote them out.
///     Mafia - A Member of the MAFIA, votes to kill players.
///     Detective - A unique Villager-Adjacent Role that can sniff out another Players Role.
///     Doctor - A unique Villager-Adjacent Role who can chose to save a Player each round.
///     Unassigned - Player has not been assigned a role yet.
///
pub enum Role {
    Villager,
    Mafia,
    Detective,
    Doctor,
    Unassigned,
}

/// Player
///
/// An instance of our Player.
///
/// TODO: I didn't want to jump down rabbit holes with code which will most likely eventually be
/// TODO: replaced. But I think we should employ typestate pattern to Control player Roles.
/// TODO: Ex: Invalid states are impossible to represent -> Impl Into and From for UnassignedPlayer
/// TODO: and Player. We ensure that only players with assigned roles can participate in the game, ensures
/// TODO: we properly assigned and unassign roles inbetween games
///
/// Args:
///     id:     The Player's unique player id.
///     name:   The Player's readible name.
///     role:   The Player's role in the Mafia game.
///     alive:  The Player's alive status.
///
pub struct Player {
    pub id: usize,
    pub name: String,
    pub role: Role,
    pub alive: bool,
}

impl Player {
    /// new()
    ///
    /// Creates a new instance of player.
    ///
    /// Args:
    ///     id: Unique usize to identify the player by.
    ///     name: The Player's name.
    ///     role: The Player's Role.
    ///
    /// Returns:
    ///     Self: a new instance of Self.
    ///
    pub fn new(id: usize, name: String, role: Role) -> Self {
        Self {
            id,
            name,
            role,
            alive: true,
        }
    }

    /// vote()
    ///
    /// Function to handle Player voting.
    /// Players can abstain a vote by passing 999.
    /// We ensure that the vote is valid and parseable.
    ///
    /// Args:
    ///  players: list of players to vote for.
    ///
    /// Returns:
    ///  Player voted for or None if no vote.
    ///
    pub fn vote<'a>(&self, players: &'a Vec<Player>) -> Option<&'a Player> {
        println!("Enter the Player id of the Player you'd like to vote for, or 999 to abstain");

        // Display voting options
        for player in players {
            println!("{} | {}", player.id, player.name);
        }

        // Accept user input
        let mut input = String::new();
        loop {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to Read Line");

            // Parse Input
            let player_id = match input.parse() {
                Ok(999)=> return None,
                Ok(id) => id,
                Err(_) => {
                    println!("{}, is not a valid vote", input);
                    continue;
                }
            };

            // Ensure the vote is a valid player and return
            if let Some(player) = players.iter().find(|p| p.id == player_id) {
                return Some(player)
            } else {
                println!("{}, is not a valid vote", input);
                continue;
            }
        }
    }

    /// act()
    ///
    /// Role-specific Round Action.
    ///
    /// ex: Villager votes, Doctor saves, etc.
    ///
    /// Args:
    ///     target: Player to target with our action.
    ///
    /// Returns:
    ///     Action: The completed Action for our Role.
    ///
    fn act(&self, target: Player) -> Option<Action> {
        match self.role {
            Role::Villager => None, // Villager has no special action
            Role::Mafia => Some(Action::Kill {
                killer: self.id,
                victim: target.id,
            }),
            Role::Detective => Some(Action::Investigate {
                sleuth: self.id,
                suspect: target.id,
            }),
            Role::Doctor => Some(Action::Save {
                doctor: self.id,
                patient: target.id,
            }),
            Role::Unassigned => panic!(), // Shouldnt be possible during a game
        }
    }
}

/// Action
///
/// Representation of a Game Action.
///
/// Variants
///     Vote: Universal action for casting a vote.
///     Kill: Mafia Action for killing a player.
///     Ivestigate: Detective Action for investigating a players role.
///     Save: Doctor Action for saving a player from a mafia kill.
///
pub enum Action {
    Vote { voter: usize, candidate: usize },
    Kill { killer: usize, victim: usize },
    Investigate { sleuth: usize, suspect: usize },
    Save { doctor: usize, patient: usize },
}
