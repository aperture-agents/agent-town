//! /src/game/player.rs
//!
//! Player module
//!
//! Contains structs and logic related to game players.
//!

use crate::game::state::GamePhase;
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
#[derive(PartialEq, Clone)]
pub enum Role {
    Villager,
    Mafia,
    Detective,
    Doctor,
    Unassigned,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Villager => write!(f, "Villager"),
            Role::Mafia => write!(f, "Mafia"),
            Role::Detective => write!(f, "Detective"),
            Role::Doctor => write!(f, "Doctor"),
            Role::Unassigned => write!(f, "Unassigned"),
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
///     Investigate: Detective Action for investigating a players role.
///     Save: Doctor Action for saving a player from a mafia kill.
///
pub enum Action {
    Vote { _voter: usize, candidate: usize },
    Kill { _killer: usize, victim: usize },
    Investigate { _sleuth: usize, suspect: usize },
    Save { _doctor: usize, patient: usize },
}

/// ActionType
///
/// Represents the types of Actions a player can make.
///
/// Variants
///     Vote: Voting for another player.
///     Kill: Mafia action to kill another player.
///     Investigate: Detective action to investigate a players role.
///     Save: Doctor action to for saving another player from a mafia kill.
pub enum ActionType {
    Vote,
    Kill,
    Investigate,
    Save,
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
#[derive(Clone)]
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
    /// We ensure that the vote is valid and parseable.
    /// Ensure that the player cannot vote for themselves.
    ///
    /// Args:
    ///  players: list of players to vote for.
    ///
    /// Returns:
    ///  Player voted for or None if no vote.
    ///
    pub fn prompt<'a>(&self, action_type: ActionType, players: &'a Vec<Player>) -> &'a Player {
        // Depending on the action display a different prompt
        match action_type {
            ActionType::Vote => println!(
                "{}, Enter the Player id of the Player you'd like to vote for",
                self.name
            ),
            ActionType::Kill => println!(
                "{}, Enter the Player id of the player you'd like to kill",
                self.name
            ),
            ActionType::Investigate => println!(
                "{} Enter the player id of the player you'd like to investigate",
                self.name
            ),
            ActionType::Save => println!(
                "{}, Enter the player id of the player you'd like to save",
                self.name
            ),
        }

        // Create new list of players minus self and dead players so players cannot vote for themselves
        let votable_players: Vec<&Player> = players
            .iter()
            .filter(|p| p.id != self.id && p.alive)
            .collect();

        // Display votable options
        for player in &votable_players {
            println!("{} | {}", player.id, player.name);
        }

        // Accept user input
        let mut input = String::new();
        loop {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to Read Line");

            // Parse Input
            let player_id = match input.trim().parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("{}, is not a valid vote", input.trim());
                    continue;
                }
            };

            // Ensure the vote is a valid player and return
            if let Some(player) = votable_players.iter().find(|p| p.id == player_id) {
                return player;
            } else {
                println!("{}, is not a valid vote", input.trim());
                continue;
            }
        }
    }

    /// act()
    ///
    /// Role-specific Round Action.
    ///
    /// ex: Villager votes, Doctor saves, etc.
    /// Players should not be able to act on themselves.
    ///
    /// Args:
    ///     target: Player to target with our action.
    ///
    /// Returns:
    ///     Action: The completed Action for our Role.
    ///
    pub fn act(&self, phase: GamePhase, players: &Vec<Player>) -> Option<Action> {
        // If were in the voting phase - all players vote
        if phase == GamePhase::Voting {
            let target = self.prompt(ActionType::Vote, players);

            return Some(Action::Vote {
                _voter: self.id,
                candidate: target.id,
            });
        }

        match self.role {
            Role::Villager => None, // Villager has no special action
            Role::Mafia => {
                // TODO: Allow some secret chat between mafia to decide on a hit.

                // Prompt for action
                let target = self.prompt(ActionType::Kill, players);

                Some(Action::Kill {
                    _killer: self.id,
                    victim: target.id,
                })
            }
            Role::Detective => {
                // Prompt for action
                let target = self.prompt(ActionType::Investigate, players);

                Some(Action::Investigate {
                    _sleuth: self.id,
                    suspect: target.id,
                })
            }
            Role::Doctor => {
                // Prompt for action
                let target = self.prompt(ActionType::Save, players);

                Some(Action::Save {
                    _doctor: self.id,
                    patient: target.id,
                })
            }
            Role::Unassigned => panic!(), // Shouldnt be possible during a game
        }
    }

    /// kill()
    ///
    /// Let the player know they have died.
    /// Eliminate them from the game by setting alive = false.
    ///
    pub fn kill(&mut self) {
        // TODO: Ping player that they were voted out
        // self.ping_dead()

        // Set the player to dead
        self.alive = false;
    }
}
