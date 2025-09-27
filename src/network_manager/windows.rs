use crate::network_manager::NetworkManager;
use windows::Win32::Foundation::{ERROR_SUCCESS, HANDLE};
use windows::Win32::NetworkManagement::WiFi::*;
use windows::core::GUID;

/// Implementation of [`NetworkManager`] for Windows OS.
pub struct WindowsNetworkManager;

impl WindowsNetworkManager {
    fn open_handle() -> Option<HANDLE> {
        let mut client_handle = HANDLE(std::ptr::null_mut());
        let mut negotiated_version = 0u32;

        let result =
            unsafe { WlanOpenHandle(2, None, &mut negotiated_version, &mut client_handle) };
        if result == ERROR_SUCCESS.0 {
            Some(client_handle)
        } else {
            println!("Failed to open WLAN handle: {}", result);
            None
        }
    }

    fn enum_interfaces(client_handle: HANDLE) -> Option<*mut WLAN_INTERFACE_INFO_LIST> {
        let mut iface_list_ptr: *mut WLAN_INTERFACE_INFO_LIST = std::ptr::null_mut();
        let result = unsafe { WlanEnumInterfaces(client_handle, None, &mut iface_list_ptr) };

        if result == ERROR_SUCCESS.0 && !iface_list_ptr.is_null() {
            Some(iface_list_ptr)
        } else {
            println!("Failed to enumerate WLAN interfaces: {}", result);
            None
        }
    }

    fn first_interface_guid(iface_list_ptr: *mut WLAN_INTERFACE_INFO_LIST) -> Option<GUID> {
        let iface_list = unsafe { &*iface_list_ptr };
        if iface_list.dwNumberOfItems == 0 {
            println!("No WLAN interfaces found");
            None
        } else {
            Some(iface_list.InterfaceInfo[0].InterfaceGuid)
        }
    }

    fn disconnect(client_handle: HANDLE, iface: &GUID) -> Option<()> {
        let result = unsafe { WlanDisconnect(client_handle, iface, None) };
        if result == ERROR_SUCCESS.0 {
            println!("Disconnected from current Wi-Fi network");
            Some(())
        } else {
            println!("WlanDisconnect failed: {result}");
            None
        }
    }

    fn connect(client_handle: HANDLE, iface: &GUID) -> Option<()> {
        todo!("Can't find defintion of WlanConnect for now.")
        // let result = WlanConnect(client_handle, iface, std::ptr::null());
        // if result == ERROR_SUCCESS.0 {
        //     println!("Reconnected successfully ✅");
        //     Some(())
        // } else {
        //     println!("Reconnect failed ❌ (error code {result})");
        //     None
        // }
    }
}

impl NetworkManager for WindowsNetworkManager {
    fn reconnect(&self) -> bool {
        Self::open_handle()
            .and_then(Self::enum_interfaces)
            .and_then(Self::first_interface_guid)
            .and_then(|guid| {
                Self::open_handle()
                    .and_then(|h| Self::disconnect(h, &guid))
                    .map(|_| guid)
            })
            .and_then(|guid| Self::open_handle().and_then(|h| Self::connect(h, &guid)))
            .is_some()
    }
}
