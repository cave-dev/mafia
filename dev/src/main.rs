use actix::dev::{MessageResponse, ResponseChannel};
use actix::fut::{err, wrap_future};
use actix::prelude::Future;
use actix::{
    Actor, ActorContext, ActorFuture, Addr, AsyncContext, Context, Handler, Message,
    ResponseActFuture, Running, StreamHandler, WeakAddr,
};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::{debug, error, info};
use mafia::{Error as MError, PlayerConnection, Response, Ruleset, Session};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

fn gen_uuid() -> String {
    let mut buffer = [0_u8; 36];
    let id = Uuid::new_v4().to_hyphenated();
    let s = id.encode_upper(&mut buffer);
    s.to_string()
}

use once_cell::sync::Lazy;

static CONNECTION_MANAGER: Lazy<Addr<ConnectionManager>> =
    Lazy::new(|| ConnectionManager::start_default());

static SESSION_MANAGER: Lazy<Addr<SessionManager>> = Lazy::new(|| SessionManager::start_default());

struct AppState {
    cm: Addr<ConnectionManager>,
    sm: Addr<SessionManager>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cm: CONNECTION_MANAGER.clone(),
            sm: SESSION_MANAGER.clone(),
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

    fn started(&mut self, _: &mut Self::Context) {
        debug!("session manager started");
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        debug!("session manager is stopping");
        Running::Stop
    }
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

struct CreateSession {
    host_name: String,
}

impl Message for CreateSession {
    type Result = CreateSessionRes;
}

impl Handler<CreateSession> for SessionManager {
    type Result = CreateSessionRes;

    fn handle(&mut self, msg: CreateSession, _: &mut Self::Context) -> Self::Result {
        let sess_id = gen_uuid();
        let host_secret = gen_uuid();
        let session = GameSession::new(sess_id.clone(), msg.host_name, host_secret.clone()).start();
        debug!("adding session {} to sessions", sess_id);
        self.sessions.insert(sess_id.clone(), session);
        CreateSessionRes {
            session_id: sess_id,
            secret: host_secret,
        }
    }
}

struct CreateSessionRes {
    session_id: String,
    secret: String,
}

impl MessageResponse<SessionManager, CreateSession> for CreateSessionRes {
    fn handle<R: ResponseChannel<CreateSession>>(
        self,
        _: &mut <SessionManager as Actor>::Context,
        tx: Option<R>,
    ) {
        if let Some(chan) = tx {
            chan.send(self)
        }
    }
}

struct SessionJoined {
    session_id: String,
    secret: String,
}

struct JoinSession {
    session_id: String,
    name: String,
}

impl Message for JoinSession {
    type Result = Result<JoinSessionRes, MError>;
}

impl Handler<JoinSession> for SessionManager {
    type Result = ResponseActFuture<Self, JoinSessionRes, MError>;

    fn handle(&mut self, msg: JoinSession, ctx: &mut Self::Context) -> Self::Result {
        let session = self.sessions.iter().find(|(k, _)| *k == &msg.session_id);
        let session_id = msg.session_id.clone();
        if let Some((_, game)) = session {
            let secret = gen_uuid();
            let create_user = game
                .send(CreateUser {
                    name: msg.name,
                    secret: secret.clone(),
                })
                .map_err(|e| MError::InternalError)
                .and_then(|res| match res {
                    Ok(_) => Ok(JoinSessionRes { session_id, secret }),
                    Err(e) => Err(e),
                });
            Box::new(wrap_future::<_, Self>(create_user))
        } else {
            Box::new(err::<_, _, Self>(MError::InvalidSession))
        }
    }
}

struct JoinSessionRes {
    session_id: String,
    secret: String,
}

struct GameSession {
    id: String,
    game: Session<ConnectionAddr>,
}

impl GameSession {
    fn new(id: String, host_name: String, host_secret: String) -> Self {
        GameSession {
            id: id.clone(),
            game: Session::new(id, Ruleset::default(), host_name, host_secret),
        }
    }
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

struct CreateUser {
    name: String,
    secret: String,
}

impl Message for CreateUser {
    type Result = Result<(), MError>;
}

impl Handler<CreateUser> for GameSession {
    type Result = Result<(), MError>;

    fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
        self.game.create_user(msg.name, msg.secret)
    }
}

#[derive(Message)]
struct RegisterConnection {
    player_name: String,
    connection: Option<ConnectionAddr>,
}

impl Handler<RegisterConnection> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: RegisterConnection, _: &mut Self::Context) -> Self::Result {
        self.game
            .register_connection(msg.player_name, msg.connection);
    }
}

struct GetPlayerConnection(String);

impl Message for GetPlayerConnection {
    type Result = Option<ConnectionAddr>;
}

impl Handler<GetPlayerConnection> for GameSession {
    type Result = Option<ConnectionAddr>;

    fn handle(&mut self, msg: GetPlayerConnection, _: &mut Self::Context) -> Self::Result {
        self.game.get_connection(&msg.0)
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
        debug!("connection {} started", self.id);
        self.cm.do_send(Connect {
            id: self.id.clone(),
            addr: ctx.address(),
        });
        let player_conn = wrap_future::<_, Self>(
            self.sess
                .send(GetPlayerConnection(self.name.clone()))
                .map_err(|_| ()),
        );
        let update = player_conn.map(|connection, act, ctx| {
            if let Some(conn_addr) = connection {
                if let Some(prev) = conn_addr.get_addr() {
                    prev.do_send(TerminateConnection);
                }
            }
            act.sess.do_send(RegisterConnection {
                player_name: act.name.clone(),
                connection: Some(ConnectionAddr(Arc::new(ctx.address().downgrade()))),
            });
        });
        ctx.spawn(update);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        debug!("connection {} terminated", self.id);
        self.cm.do_send(Disconnect {
            id: self.id.clone(),
        });
        let player_conn = wrap_future::<_, Self>(
            self.sess
                .send(GetPlayerConnection(self.name.clone()))
                .map_err(|_| ()),
        );
        let unregister = player_conn.map(|connection, act, ctx| {
            let mut should_unregister = false;
            if let Some(conn_addr) = connection {
                if let Some(prev) = conn_addr.get_addr() {
                    if prev == ctx.address() {
                        should_unregister = true;
                    }
                }
            } else {
                should_unregister = true;
            }

            if should_unregister {
                act.sess.do_send(RegisterConnection {
                    player_name: act.name.clone(),
                    connection: None,
                });
            };
        });
        ctx.spawn(unregister);
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
struct TerminateConnection;

impl Handler<TerminateConnection> for Connection {
    type Result = ();

    fn handle(&mut self, msg: TerminateConnection, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
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

impl ConnectionAddr {
    fn is_alive(&self) -> bool {
        self.get_addr().is_some()
    }

    fn get_addr(&self) -> Option<Addr<Connection>> {
        match self.0.upgrade() {
            Some(c) => Some(c),
            None => None,
        }
    }
}

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
struct CreateLobbyArgs {
    name: String,
}

#[derive(Serialize)]
struct CreateLobbyResp {
    session_id: String,
    secret: String,
}

fn create_lobby(
    data: web::Data<AppState>,
    web::Query(info): web::Query<CreateLobbyArgs>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    data.sm
        .send(CreateSession {
            host_name: info.name,
        })
        .map_err(|e| Error::from(e))
        .and_then(|resp| {
            Ok(HttpResponse::Ok().json(CreateLobbyResp {
                session_id: resp.session_id,
                secret: resp.secret,
            }))
        })
}

#[derive(Deserialize)]
struct JoinLobbyArgs {
    session_id: String,
    name: String,
}

#[derive(Serialize)]
struct JoinLobbyRes {
    session_id: String,
    secret: String,
}

fn join_lobby(
    data: web::Data<AppState>,
    web::Query(info): web::Query<JoinLobbyArgs>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    data.sm
        .send(JoinSession {
            session_id: info.session_id,
            name: info.name,
        })
        .map_err(|e| Error::from(e))
        .and_then(|resp| match resp {
            Ok(res) => Ok(HttpResponse::Ok().json(JoinLobbyRes {
                session_id: res.session_id,
                secret: res.secret,
            })),
            Err(e) => Err(Error::from(InternalError::new(
                e,
                StatusCode::from_u16(500).unwrap(),
            ))),
        })
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
    debug!(
        "websocket connect: session={} secret={}",
        info.session, info.secret
    );
    data.sm
        .send(GetSession {
            id: info.session.clone(),
        })
        .map_err(|e| {
            error!("error getting session: {}", e);
            Error::from(e)
        })
        .and_then(move |sess| {
            let game_sess = match sess {
                Some(game_sess) => game_sess,
                None => {
                    error!("no session found: {}", info.session);
                    return Err(Error::from(InternalError::new(
                        MError::InvalidSession,
                        StatusCode::from_u16(500).unwrap(),
                    )));
                }
            };

            Ok(game_sess
                .send(GetPlayername {
                    secret: info.secret.clone(),
                })
                .map_err(|e| {
                    error!("error getting player name: {}", e);
                    Error::from(e)
                })
                .and_then(move |playername_res| match playername_res {
                    Some(name) => {
                        debug!("session and player found, registering websocket connection: session={} player={}", info.session, name);
                        ws::start(
                        Connection::new(name, data.cm.clone(), game_sess),
                        &req,
                        stream,
                    )},
                    None => {
                        error!("player secret not found: sess={} secret={}", info.session, info.secret);
                        Err(Error::from(InternalError::new(
                        MError::InvalidSecret,
                        StatusCode::from_u16(500).unwrap(),
                    )))},
                }))
        })
        .flatten()
}

fn main() {
    std::env::set_var("RUST_LOG", "dev=debug,mafia=debug,actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(AppState::default())
            .route("/create", web::get().to_async(create_lobby))
            .route("/join", web::get().to_async(create_lobby))
            .route("/ws", web::get().to_async(connect_websocket))
            .wrap(Logger::new("ip=%a code=%r req_mili=%D resp_size=%b"))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
