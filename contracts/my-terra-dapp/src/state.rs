use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub enum FieldState {
    Empty,
    X,
    O,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub enum GameState {
    InProgress,
    XWon,
    OWon,
    Draw,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub fields: [FieldState; 9],
    pub game_state: GameState,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
