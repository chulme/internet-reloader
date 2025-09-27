/// Trait for managing network connections.
pub trait NetworkManager {
    /// Attempts to reconnect to the network.
    fn reconnect(&self) -> bool;
}
