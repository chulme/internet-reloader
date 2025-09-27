use crate::internet_connectivity::InternetConnectivity;
use windows::Win32::Networking::WinInet::{FLAG_ICC_FORCE_CONNECTION, InternetCheckConnectionW};
use windows::Win32::Networking::WinInet::{INTERNET_CONNECTION, InternetGetConnectedState};

/// Implementation of [`InternetConnectivity`] for Windows OS.
pub struct WindowsInternetConnectivity;

impl InternetConnectivity for WindowsInternetConnectivity {
    fn is_connected_to_network(&self) -> bool {
        let mut flags = INTERNET_CONNECTION(0);
        unsafe { InternetGetConnectedState(&mut flags, Some(0)).is_ok() }
    }

    fn is_connected_to_internet(&self) -> bool {
        let url: Vec<u16> = "http://www.google.com"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        unsafe {
            InternetCheckConnectionW(
                windows::core::PCWSTR(url.as_ptr()),
                FLAG_ICC_FORCE_CONNECTION,
                0,
            )
            .is_ok()
        }
    }
}
