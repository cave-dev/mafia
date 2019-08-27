use crate::core::night_action::NightAction;
use crate::core::session::Session;

pub type PlayerName = String;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Alive,
    Dead,
}

impl PlayerState {
    pub fn is_alive(self) -> bool {
        self == PlayerState::Alive
    }

    pub fn is_dead(self) -> bool {
        self == PlayerState::Dead
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Alive
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlayerRole {
    Townie,
    Mafioso,
    Doctor,
    Detective,
    Bartender,
}

impl PlayerRole {
    pub fn is_town(self) -> bool {
        use PlayerRole::*;
        match self {
            Townie | Doctor | Detective | Bartender => true,
            _ => false,
        }
    }

    pub fn is_mafia(self) -> bool {
        use PlayerRole::*;
        match self {
            Mafioso => true,
            _ => false,
        }
    }

    pub fn valid_night_action(self, action: &NightAction) -> bool {
        use NightAction::*;
        use PlayerRole::*;
        match action {
            None => true,
            Investigate(_) => self == Detective,
            Save(_) => self == Doctor,
            Negate(_) => self == Bartender,
            Vote(_) => self.is_mafia(),
        }
    }
}

impl Default for PlayerRole {
    fn default() -> Self {
        PlayerRole::Townie
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Player {
    pub name: PlayerName,
    pub session: Session,
    pub state: PlayerState,
    pub role: PlayerRole,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            state: PlayerState::default(),
            session: Session::default(),
            role: PlayerRole::default(),
        }
    }
}
