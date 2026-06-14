//! /src/game/player.rs
//!
//! Player module
//!
//! Contains structs and logic related to game players.

/// Player
///
/// An instance of our Player.
///
/// Args:
///     id:     The Player's unique player id.
///     name:   The Player's readible name.
///     role:   The Player's role in the Mafia game.
///     alive:  The Player's alive status.
pub struct Player {
    id: u32,
    name: String,
    role: Role,
    alive: bool,
}

impl Player {
    /// new()
    ///
    /// Creates a new instance of player.
    ///
    pub fn new(id: u32, name: String, role: Role, alive: bool) -> Self {
        Self {
            id,
            name,
            role,
            alive
        }
    }
}

/// Role
///
/// The various Player Roles in Mafia, Roles dictate a Player's actions in the game
///
/// Roles:
///     VILLAGER - They are 'innocent', works to find MAFIA members and vote them out.
///     MAFIA - A Member of the MAFIA, votes to kill players.
///     DETECTIVE - A unique VILLAGER-Adjacent Role that can sniff out another Players Role.
///     DOCTOR - A unique VILLAGER-Adjacent Role who can chose to save a Player each round.
///
pub enum Role {
    VILLAGER,
    MAFIA,
    DETECTIVE,
    DOCTOR
}

impl Role {
    /// act()
    ///
    /// Role-specific Round Action.
    ///
    /// ex: VILLAGER votes, DOCTOR saves, etc.
    ///
    fn act(&self) {
        match self {
            Role::VILLAGER => todo!(),
            Role::MAFIA => todo!(),
            Role::DETECTIVE => todo!(),
            Role::DOCTOR => todo!(),
        }
    }
}
