use crate::core::player::Player;
use crate::core::session::Session;
use im::Vector;

#[derive(Debug, Clone)]
pub struct LobbyState {
    players: Vector<Player>,
}

impl LobbyState {
    pub fn connect(mut self, player_name: &str, s: Session) -> Self {
        if let Some(p) = self.players.iter_mut().find(|p| p.name == player_name) {
            *p = Player {
                session: s,
                ..p.clone()
            };
        } else {
            self.players.push_back(Player::new(player_name.to_string()));
        }
        self
    }
}

impl Default for LobbyState {
    fn default() -> Self {
        LobbyState {
            players: Vector::new(),
        }
    }
}
