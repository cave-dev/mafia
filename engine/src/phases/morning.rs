use crate::action::ActionE;
use crate::error::Result;
use crate::player::PlayerName;
use crate::player_connection::PlayerConnection;
use crate::state::RootState;
use crate::util::message_basic;
use im::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Morning {}

impl Morning {
    pub fn handle_action<PC>(
        self,
        root: RootState<PC>,
        player: PlayerName,
        act: ActionE,
    ) -> Result<(Self, RootState<PC>)>
    where
        PC: PlayerConnection,
    {
        match act {
            ActionE::Message(m) => message_basic(&player, m, || root.players.iter())?,
        }
        Ok((self, root))
    }

    pub fn next_phase<PC>(self, root: RootState<PC>) -> (Self, RootState<PC>)
    where
        PC: PlayerConnection,
    {
        (
            self,
            RootState {
                vote_skip: HashSet::new(),
                next_state_time: root.rules.vote_end(),
                ..root
            },
        )
    }
}
