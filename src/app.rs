use crate::internet_connectivity::InternetConnectivity;
use crate::network_manager::NetworkManager;

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkStatus {
    Connected,
    NetworkOnly,
    Disconnected,
}
pub struct NetworkApp<C: InternetConnectivity, M: NetworkManager> {
    checker: C,
    manager: M,
}

impl<C: InternetConnectivity, M: NetworkManager> NetworkApp<C, M> {
    pub fn new(checker: C, manager: M) -> Self {
        return Self { checker, manager };
    }

    pub fn poll(&self) -> NetworkStatus {
        match (
            self.checker.is_connected_to_network(),
            self.checker.is_connected_to_internet(),
        ) {
            (true, true) => return NetworkStatus::Connected,
            (false, _) => return NetworkStatus::Disconnected,
            (true, false) => {
                println!("Network connected, but no internet, attempting reconnect...");
                let success = self.manager.reconnect();

                match success {
                    true => {
                        println!("Reconnected successfully");
                        return NetworkStatus::Connected;
                    }
                    false => {
                        println!("Reconnect failed");
                        return NetworkStatus::NetworkOnly;
                    }
                }
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        use crate::internet_connectivity::WindowsInternetConnectivity;
        use crate::network_manager::WindowsNetworkManager;

        impl Default for NetworkApp<WindowsInternetConnectivity, WindowsNetworkManager> {
            fn default() -> Self {
                Self::new(WindowsInternetConnectivity {}, WindowsNetworkManager {})
            }
        }
    } else {
        compile_error!("Unsupported OS: this crate only supports Windows.");
    }
}
