use crate::core::lobby_state::LobbyState;
use crate::core::night_action::NightAction;
use crate::core::player::Player;
use crate::core::playing_state::PlayingState;
use crate::core::rules::Rules;
use crate::core::{Error, PlayerName, Result};
use im::Vector;

pub type Day = u32;

#[derive(Debug, Clone)]
enum State {
    Lobby(LobbyState),
    Playing(PlayingState),
}

#[derive(Debug, Clone)]
pub struct Game {
    players: Vector<Player>,
    state: State,
    rules: Rules,
}

impl Game {
    fn take_night_action(self, player_name: PlayerName, action: NightAction) -> Result<Self> {
        let player = match self.players.iter().find(|p| p.name == player_name) {
            Some(player) => player,
            None => return Err(Error::NameNotFound(player_name)),
        };

        if !player.role.valid_night_action(&action) {
            return Err(Error::InvalidNightAction(player.role, action));
        }

        Ok(Game {
            state: match &self.state {
                State::Playing(PlayingState::Night { actions, ends_at }) => {
                    State::Playing(PlayingState::Night {
                        actions: actions.update(player_name, action),
                        ends_at: *ends_at,
                    })
                }
                _ => self.state,
            },
            ..self
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        let rules = Rules::default();
        Game {
            players: Vector::new(),
            state: State::Lobby(LobbyState::default()),
            rules,
        }
    }
}
