use crate::core::player::Player;
use im::Vector;

#[derive(Debug, Clone)]
pub struct LobbyState {
    players: Vector<Player>,
}

impl Default for LobbyState {
    fn default() -> Self {
        LobbyState {
            players: Vector::new(),
        }
    }
}
