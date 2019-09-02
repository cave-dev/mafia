use crate::response::Response;

pub trait PlayerConnection: Clone {
    fn send(&self, r: Response);
    fn is_alive(&self) -> bool;
}
