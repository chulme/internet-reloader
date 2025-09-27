use crate::network_manager::NetworkManager;
use windows::Win32::Foundation::{ERROR_SUCCESS, HANDLE};
use windows::Win32::NetworkManagement::WiFi::*;
use windows::core::{GUID, PCWSTR};

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

    fn get_network_interface(handle: HANDLE) -> Option<GUID> {
        Self::enum_interfaces(handle).and_then(|iface_list_ptr: *mut WLAN_INTERFACE_INFO_LIST| {
            let guid = Self::first_interface_guid(iface_list_ptr);
            // Clean up the interface list after extracting the GUID
            unsafe { WlanFreeMemory(iface_list_ptr as _) };
            guid
        })
    }

    fn get_current_profile_name(handle: HANDLE, iface: &GUID) -> Option<String> {
        let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        let mut data_size: u32 = 0;
        let mut opcode: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(0);

        let result = unsafe {
            WlanQueryInterface(
                handle,
                iface,
                wlan_intf_opcode_current_connection,
                None,
                &mut data_size,
                &mut data_ptr,
                Some(&mut opcode),
            )
        };

        if result != ERROR_SUCCESS.0 || data_ptr.is_null() {
            return None;
        }

        let conn_info = unsafe { &*(data_ptr as *const WLAN_CONNECTION_ATTRIBUTES) };
        let profile_name = String::from_utf16_lossy(&conn_info.strProfileName)
            .trim_end_matches('\0')
            .to_string();

        unsafe { WlanFreeMemory(data_ptr as *mut _) };
        Some(profile_name)
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
    fn connect(handle: HANDLE, iface: &GUID, profile: &str) -> Option<()> {
        // Convert profile string to PCWSTR
        let utf16: Vec<u16> = profile.encode_utf16().chain(Some(0)).collect();
        let profile_pcwstr = PCWSTR(utf16.as_ptr());

        let result = unsafe {
            WlanConnect(
                handle,
                iface,
                &WLAN_CONNECTION_PARAMETERS {
                    wlanConnectionMode: wlan_connection_mode_profile,
                    strProfile: profile_pcwstr,
                    pDot11Ssid: std::ptr::null_mut(),
                    pDesiredBssidList: std::ptr::null_mut(),
                    dot11BssType: dot11_BSS_type_any,
                    dwFlags: 0,
                },
                None,
            )
        };

        if result == ERROR_SUCCESS.0 {
            println!("Reconnect initiated");
            Some(())
        } else {
            println!("Reconnect failed (error code {result})");
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
}

impl NetworkManager for WindowsNetworkManager {
    fn reconnect(&self) -> bool {
        Self::open_handle()
            .and_then(|handle| Self::get_network_interface(handle).map(|guid| (handle, guid)))
            .and_then(|(handle, guid)| {
                // Get profile name for this interface
                let profile_name = Self::get_current_profile_name(handle, &guid)?;
                Some((handle, guid, profile_name))
            })
            .and_then(|(handle, guid, profile)| {
                // Disconnect first
                let _ = Self::disconnect(handle, &guid);
                Some((handle, guid, profile))
            })
            .and_then(|(handle, guid, profile)| {
                // Connect using the saved profile
                let success = Self::connect(handle, &guid, &profile);
                unsafe { WlanCloseHandle(handle, None) };
                success
            })
            .is_some()
    }
}
