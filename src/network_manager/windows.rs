use crate::network_manager::NetworkManager;

pub struct WindowsNetworkManager;

impl NetworkManager for WindowsNetworkManager {
    fn reconnect(&self) -> bool {
        return true;
    }
}
