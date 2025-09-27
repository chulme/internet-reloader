//! Module for checking internet connectivity.

mod interface;
mod windows;

pub use interface::InternetConnectivity;
pub use windows::WindowsInternetConnectivity;
