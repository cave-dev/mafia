use crate::action::ActionMessage;
use crate::error::{Error, Result};
use crate::player::{Player, PlayerNameRef};
use crate::player_connection::PlayerConnection;
use crate::response::Response;
use chrono::Duration;
use serde::{Deserialize, Deserializer, Serializer};
use std::result::Result as StdResult;

pub fn se_opt_dur<S: Serializer>(dur: &Option<Duration>, s: S) -> StdResult<S::Ok, S::Error> {
    match dur {
        Some(d) => s.serialize_some(&d.num_seconds()),
        None => s.serialize_none(),
    }
}

pub fn de_opt_dur<'de, D: Deserializer<'de>>(d: D) -> StdResult<Option<Duration>, D::Error> {
    Ok(Option::deserialize(d)?.map(Duration::seconds))
}

pub fn message_if<'a, PC, I, P, C>(
    sender: PlayerNameRef,
    message: ActionMessage,
    players: P,
    cond: C,
) -> Result<()>
where
    PC: PlayerConnection + 'a,
    I: Iterator<Item = &'a Player<PC>>,
    P: Fn() -> I,
    C: Fn(&Player<PC>, &Player<PC>) -> bool,
{
    let src: &Player<PC> = players()
        .filter(|p| p.get_name() == sender)
        .map(Ok)
        .next()
        .unwrap_or_else(|| Err(Error::InvalidPlayerName(sender.to_string())))?;

    let targets = players().filter(|dest| cond(src, dest));

    for target in targets {
        if let Some(conn) = &target.connection {
            conn.send(Response::message(
                Some(src.get_name().to_string()),
                message.text.clone(),
            ))
        }
    }
    Ok(())
}

pub fn message_basic<'a, PC, I, P>(
    sender: PlayerNameRef,
    message: ActionMessage,
    players: P,
) -> Result<()>
where
    PC: PlayerConnection + 'a,
    I: Iterator<Item = &'a Player<PC>>,
    P: Fn() -> I,
{
    fn cond<PC: PlayerConnection>(src: &Player<PC>, dest: &Player<PC>) -> bool {
        src.state.is_alive() || !dest.state.is_alive()
    }
    message_if(sender, message, players, cond)
}
