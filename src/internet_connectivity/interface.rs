pub trait InternetConnectivity {
    fn is_connected_to_network(&self) -> bool;
    fn is_connected_to_internet(&self) -> bool;
}
