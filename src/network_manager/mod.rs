//! Module for managing network connections.

mod interface;
mod windows;

pub use interface::NetworkManager;
pub use windows::WindowsNetworkManager;
