//! Module for the main application logic.
//!
//! This module defines the [`NetworkApp`] struct, which utilizes the [`InternetConnectivity`] and [`NetworkManager`] traits to monitor and manage network connectivity.
//! ! It provides functionality to poll the network status and attempt reconnections when necessary.

use crate::internet_connectivity::InternetConnectivity;
use crate::network_manager::NetworkManager;

/// Represents the network status.
#[derive(Debug, PartialEq, Eq)]
pub enum NetworkStatus {
    /// Connected to both network and internet
    Connected,

    /// Connected to network but not internet
    NetworkOnly,

    /// Not connected to network
    Disconnected,
}

/// Implementing the [`std::fmt::Display`] trait for [`NetworkStatus`] to enable easy printing.
impl std::fmt::Display for NetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NetworkStatus::Connected => write!(f, "Connected"),
            NetworkStatus::NetworkOnly => write!(f, "NetworkOnly"),
            NetworkStatus::Disconnected => write!(f, "Disconnected"),
        }
    }
}

/// The main application struct that uses the [`InternetConnectivity`] and [`NetworkManager`] traits.
///
/// # Type Parameters
/// - `C`: A type that implements the [`InternetConnectivity`] trait.
/// - `M`: A type that implements the [`NetworkManager`] trait.
pub struct NetworkApp<C: InternetConnectivity, M: NetworkManager> {
    checker: C,
    manager: M,
}

/// Implementation of the [`NetworkApp`] struct.
impl<C: InternetConnectivity, M: NetworkManager> NetworkApp<C, M> {
    /// Creates a new instance of [`NetworkApp`]`.
    ///
    /// # Arguments
    /// - `checker`: An instance of a type that implements the [`InternetConnectivity`] trait.
    /// - `manager`: An instance of a type that implements the [`NetworkManager`] trait
    pub fn new(checker: C, manager: M) -> Self {
        Self { checker, manager }
    }

    /// Polls the network status and attempts to reconnect if necessary.
    ///
    /// Returns the current [`NetworkStatus`].
    pub fn poll(&self) -> NetworkStatus {
        match (
            self.checker.is_connected_to_network(),
            self.checker.is_connected_to_internet(),
        ) {
            (true, true) => NetworkStatus::Connected,
            (false, _) => NetworkStatus::Disconnected,
            (true, false) => {
                println!("Network connected, but no internet, attempting reconnect...");
                let success = self.manager.reconnect();

                match success {
                    true => {
                        println!("Reconnected successfully");
                        NetworkStatus::Connected
                    }
                    false => {
                        println!("Reconnect failed");
                        NetworkStatus::NetworkOnly
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

        /// Default implementation of [`NetworkApp`] for Windows OS.
        impl Default for NetworkApp<WindowsInternetConnectivity, WindowsNetworkManager> {
            fn default() -> Self {
                Self::new(WindowsInternetConnectivity {}, WindowsNetworkManager {})
            }
        }
    } else {
        compile_error!("Unsupported OS: this crate only supports Windows.");
    }
}
