//! A Rust application to monitor internet connectivity, and if no internet available, attempt to reconnect the network.
//!
//! This is currently only implemented for Windows OS, as an over-engineered method of resolving internet connectivity issues
//! when using my personal phone's hotspot for connectivity.

pub mod app;
pub mod internet_connectivity;
pub mod network_manager;
