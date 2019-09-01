use crate::response::Response;

pub trait PlayerConnection: Clone {
    fn send(&self, r: Response);
    fn id(&self) -> &str;
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
