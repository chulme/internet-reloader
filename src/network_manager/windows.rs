use crate::network_manager::NetworkManager;

/// Implementation of [`NetworkManager`] for Windows OS.
pub struct WindowsNetworkManager;

impl NetworkManager for WindowsNetworkManager {
    fn reconnect(&self) -> bool {
        true
    }
}
