use actix::prelude::Future;
use actix::{
    Actor, Addr, AsyncContext, Context, Handler, Message, Running, StreamHandler, WeakAddr,
};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use mafia::{Error as MError, PlayerConnection, Response, Session};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

fn gen_uuid() -> String {
    let mut buffer = [0_u8; 36];
    let id = Uuid::new_v4().to_hyphenated();
    let s = id.encode_upper(&mut buffer);
    s.to_string()
}

struct AppState {
    cm: Addr<ConnectionManager>,
    sm: Addr<SessionManager>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cm: ConnectionManager::default().start(),
            sm: SessionManager::default().start(),
        }
    }
}

struct ConnectionManager {
    connections: HashMap<String, Addr<Connection>>,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        ConnectionManager {
            connections: HashMap::new(),
        }
    }
}

impl Actor for ConnectionManager {
    type Context = Context<Self>;
}

#[derive(Message)]
struct Connect {
    id: String,
    addr: Addr<Connection>,
}

impl Handler<Connect> for ConnectionManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.connections.insert(msg.id, msg.addr);
    }
}

#[derive(Message)]
struct Disconnect {
    id: String,
}

impl Handler<Disconnect> for ConnectionManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.connections.remove(&msg.id);
    }
}

struct SessionManager {
    sessions: HashMap<String, Addr<GameSession>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        SessionManager {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for SessionManager {
    type Context = Context<Self>;
}

#[derive(Message)]
struct RegisterSession {
    id: String,
    addr: Addr<GameSession>,
}

impl Handler<RegisterSession> for SessionManager {
    type Result = ();

    fn handle(&mut self, msg: RegisterSession, _: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.id, msg.addr);
    }
}

struct GetSession {
    id: String,
}

impl Message for GetSession {
    type Result = Option<Addr<GameSession>>;
}

impl Handler<GetSession> for SessionManager {
    type Result = Option<Addr<GameSession>>;

    fn handle(&mut self, msg: GetSession, _: &mut Self::Context) -> Self::Result {
        self.sessions.get(&msg.id).map(|v| v.clone())
    }
}

struct GameSession {
    game: Session<ConnectionAddr>,
}

impl Actor for GameSession {
    type Context = Context<Self>;
}

struct GetPlayername {
    secret: String,
}

impl Message for GetPlayername {
    type Result = Option<String>;
}

impl Handler<GetPlayername> for GameSession {
    type Result = Option<String>;

    fn handle(&mut self, msg: GetPlayername, _: &mut Self::Context) -> Self::Result {
        self.game.get_playername(&msg.secret)
    }
}

#[derive(Clone)]
struct Connection {
    id: String,
    name: String,
    cm: Addr<ConnectionManager>,
    sess: Addr<GameSession>,
}

impl Connection {
    fn new(name: String, cm: Addr<ConnectionManager>, sess: Addr<GameSession>) -> Self {
        Connection {
            id: gen_uuid(),
            name,
            cm,
            sess,
        }
    }
}

impl Actor for Connection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.cm.do_send(Connect {
            id: self.id.clone(),
            addr: ctx.address(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.cm.do_send(Disconnect {
            id: self.id.clone(),
        });
        Running::Stop
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Connection {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            _ => (),
        }
    }
}

#[derive(Message)]
struct SendMsg(String);

impl Handler<SendMsg> for Connection {
    type Result = ();

    fn handle(&mut self, msg: SendMsg, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

#[derive(Clone)]
struct ConnectionAddr(Arc<WeakAddr<Connection>>);

impl PlayerConnection for ConnectionAddr {
    fn send(&self, r: Response) {
        if let Some(addr) = self.0.upgrade() {
            addr.do_send(SendMsg(
                serde_json::to_string(&r).expect("error serializing response"),
            ))
        }
    }

    fn is_alive(&self) -> bool {
        self.0.upgrade().is_some()
    }
}

#[derive(Deserialize)]
pub struct WebsocketAuth {
    session: String,
    secret: String,
}

fn connect_websocket(
    data: web::Data<AppState>,
    web::Query(info): web::Query<WebsocketAuth>,
    req: HttpRequest,
    stream: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    data.sm
        .send(GetSession {
            id: info.session.clone(),
        })
        .map_err(|e| Error::from(e))
        .and_then(move |sess| {
            let game_sess = match sess {
                Some(game_sess) => game_sess,
                None => {
                    return Err(Error::from(InternalError::new(
                        MError::InvalidSession,
                        StatusCode::from_u16(500).unwrap(),
                    )))
                }
            };

            Ok(game_sess
                .send(GetPlayername {
                    secret: info.secret,
                })
                .map_err(|e| Error::from(e))
                .and_then(move |playername_res| match playername_res {
                    Some(name) => ws::start(
                        Connection::new(name, data.cm.clone(), game_sess),
                        &req,
                        stream,
                    ),
                    None => Err(Error::from(InternalError::new(
                        MError::InvalidSecret,
                        StatusCode::from_u16(500).unwrap(),
                    ))),
                }))
        })
        .flatten()
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(AppState::default())
            .route("/ws", web::get().to_async(connect_websocket))
            .wrap(Logger::new("ip=%a code=%r req_mili=%D resp_size=%b"))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
