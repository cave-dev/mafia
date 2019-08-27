use crate::core::PlayerName;

#[derive(Clone, Debug)]
pub enum NightAction {
    None,
    Save(PlayerName),
    Investigate(PlayerName),
    Negate(PlayerName),
    Vote(PlayerName),
}

impl Default for NightAction {
    fn default() -> Self {
        NightAction::None
    }
}
