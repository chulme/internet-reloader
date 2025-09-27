/// Trait for checking internet connectivity.
pub trait InternetConnectivity {
    /// Checks if the system is connected to a network.
    fn is_connected_to_network(&self) -> bool;

    /// Checks if the system is connected to the internet.
    fn is_connected_to_internet(&self) -> bool;
}
