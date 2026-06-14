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
#[expect(dead_code)] // Not Used Yet - dead_code
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
    #[expect(dead_code)] // Not Used Yet - dead_code
    pub fn new(id: u32, name: String, role: Role, alive: bool) -> Self {
        Self {
            id,
            name,
            role,
            alive,
        }
    }
}

/// Role
///
/// The various Player Roles in Mafia, Roles dictate a Player's actions in the game
///
/// Roles:
///     Villager - They are 'innocent', works to find Mafia members and vote them out.
///     Mafia - A Member of the MAFIA, votes to kill players.
///     Detective - A unique Villager-Adjacent Role that can sniff out another Players Role.
///     Doctor - A unique Villager-Adjacent Role who can chose to save a Player each round.
///
#[expect(dead_code)] // Not Used Yet - dead_code
pub enum Role {
    Villager,
    Mafia,
    Detective,
    Doctor,
}

impl Role {
    /// act()
    ///
    /// Role-specific Round Action.
    ///
    /// ex: Villager votes, Doctor saves, etc.
    ///
    #[expect(dead_code)] // Not Used Yet - DeadCode
    fn act(&self) {
        match self {
            Role::Villager => todo!(),
            Role::Mafia => todo!(),
            Role::Detective => todo!(),
            Role::Doctor => todo!(),
        }
    }
}
