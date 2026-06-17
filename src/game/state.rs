//! /src/game/state.rs
//!
//! State module.
//!
//! Contains structs and logic related to maintaining the game state.
//!


use crate::game::player::{Player, Action, Vote};
use crate::game::chat::Chat;

/// GamePhase
///
/// An Enum for the various game phases.
///
/// Variants:
///     Discussion: Phase when players discuss what to do.
///     Voting: Short Phase when players vote out a player.
///     Night: Short Phase when unique roles chose their targets.
///

enum GamePhase {
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
struct GameState {
    phase: GamePhase,
    players: Vec<Player>,
    prev_rounds: Vec<Round>
}

/// Round
///
/// A Round contains information for each given game round.
///
/// Args:
///     chat:   The chat instance for this round.
///     votes:  The voting record for this round.
///     acts:   The record of actions for this round using a trait object here.
///
struct Round {
    chat: Chat,
    votes: Vec<Vote>,
    acts: Vec<Box<dyn Action>>,
}
