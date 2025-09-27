use crate::internet_connectivity::InternetConnectivity;
use windows::Win32::Networking::WinInet::{INTERNET_CONNECTION, InternetGetConnectedState};

pub struct WindowsInternetConnectivity;

impl InternetConnectivity for WindowsInternetConnectivity {
    fn is_connected_to_network(&self) -> bool {
        let mut flags = INTERNET_CONNECTION(0);
        unsafe {
            return InternetGetConnectedState(&mut flags, Some(0)).is_ok();
        }
    }

    fn is_connected_to_internet(&self) -> bool {
        return true;
    }
}
