//! /src/game/player.rs
//!
//! Player module
//!
//! Contains structs and logic related to game players.
//!

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
/// Args:
///     id:     The Player's unique player id.
///     name:   The Player's readible name.
///     role:   The Player's role in the Mafia game.
///     alive:  The Player's alive status.
///
pub struct Player {
    id: usize,
    name: String,
    pub role: Role,
    alive: bool,
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
    fn act(&self, target: Player) -> Action {
        match self.role {
            Role::Villager => Action::Vote {
                voter: self.id,
                candidate: target.id,
            },
            Role::Mafia => Action::Kill {
                killer: self.id,
                victim: target.id,
            },
            Role::Detective => Action::Investigate {
                sleuth: self.id,
                suspect: target.id,
            },
            Role::Doctor => Action::Save {
                doctor: self.id,
                patient: target.id,
            },
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
