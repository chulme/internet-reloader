pub trait NetworkManager {
    fn reconnect(&self) -> bool;
}
