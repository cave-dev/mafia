#[derive(Clone, Debug, Hash)]
pub enum Session {
    Connected { session_id: String },
    Disconnected,
}

impl Default for Session {
    fn default() -> Self {
        Session::Disconnected
    }
}
