use std::ffi::c_void;

use crate::network_manager::NetworkManager;
use windows::Win32::Foundation::{ERROR_SUCCESS, HANDLE};
use windows::Win32::NetworkManagement::WiFi::*;
use windows::core::{GUID, PCWSTR};

/// Trait to abstract Windows WLAN API calls.
pub trait WlanApi {
    /// Opens a handle to the WLAN API.
    ///
    /// # Safety
    /// This function is unsafe because it involves raw pointers and FFI calls.
    unsafe fn open_handle(
        dwclientversion: u32,
        preserved: Option<*const core::ffi::c_void>,
        pdwnegotiatedversion: *mut u32,
        phclienthandle: *mut windows::Win32::Foundation::HANDLE,
    ) -> u32;

    /// Closes a handle to the WLAN API.
    ///
    /// # Safety
    /// This function is unsafe because it involves raw pointers and FFI calls.
    unsafe fn close_handle(handle: HANDLE, preserved: Option<*const c_void>) -> u32;

    /// Frees memory allocated by the WLAN API.
    ///
    /// # Safety
    /// This function is unsafe because it involves raw pointers and FFI calls.
    unsafe fn free_memory(pmemory: *const core::ffi::c_void);

    /// Queries information about the specified wireless LAN interface.
    ///
    /// # Safety
    /// This function is unsafe because it involves raw pointers and FFI calls.
    unsafe fn query_interface(
        hclienthandle: windows::Win32::Foundation::HANDLE,
        pinterfaceguid: *const windows_core::GUID,
        opcode: WLAN_INTF_OPCODE,
        preserved: Option<*const core::ffi::c_void>,
        pdwdatasize: *mut u32,
        ppdata: *mut *mut core::ffi::c_void,
        pwlanopcodevaluetype: Option<*mut WLAN_OPCODE_VALUE_TYPE>,
    ) -> u32;

    /// Connects the specified wireless LAN interface to a network.
    ///
    /// # Safety
    /// This function is unsafe because it involves raw pointers and FFI calls.
    unsafe fn connect(
        hclienthandle: windows::Win32::Foundation::HANDLE,
        pinterfaceguid: *const windows_core::GUID,
        pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS,
        preserved: Option<*const core::ffi::c_void>,
    ) -> u32;

    /// Disconnects the specified wireless LAN interface from the current network.
    ///
    /// # Safety
    /// This function is unsafe because it involves raw pointers and FFI calls.
    unsafe fn disconnect(
        hclienthandle: windows::Win32::Foundation::HANDLE,
        pinterfaceguid: *const windows_core::GUID,
        preserved: Option<*const core::ffi::c_void>,
    ) -> u32;

    /// Enumerates the wireless LAN interfaces on the local computer.
    ///
    /// # Safety
    /// This function is unsafe because it involves raw pointers and FFI calls.
    unsafe fn enum_interfaces(
        hclienthandle: windows::Win32::Foundation::HANDLE,
        preserved: Option<*const core::ffi::c_void>,
        ppinterfacelist: *mut *mut WLAN_INTERFACE_INFO_LIST,
    ) -> u32;
}

/// Concrete implementation of the [`WlanApi`] trait using actual Windows API calls.
pub struct WlanApiImpl;

#[no_coverage]
impl WlanApi for WlanApiImpl {
    // All methods implement the real Windows API logic, as in your current code
    unsafe fn open_handle(
        dwclientversion: u32,
        preserved: Option<*const core::ffi::c_void>,
        pdwnegotiatedversion: *mut u32,
        phclienthandle: *mut windows::Win32::Foundation::HANDLE,
    ) -> u32 {
        unsafe {
            WlanOpenHandle(
                dwclientversion,
                preserved,
                pdwnegotiatedversion,
                phclienthandle,
            )
        }
    }

    unsafe fn close_handle(handle: HANDLE, preserved: Option<*const c_void>) -> u32 {
        unsafe { WlanCloseHandle(handle, preserved) }
    }
    unsafe fn free_memory(pmemory: *const core::ffi::c_void) {
        unsafe { WlanFreeMemory(pmemory) }
    }

    unsafe fn query_interface(
        hclienthandle: windows::Win32::Foundation::HANDLE,
        pinterfaceguid: *const windows_core::GUID,
        opcode: WLAN_INTF_OPCODE,
        preserved: Option<*const core::ffi::c_void>,
        pdwdatasize: *mut u32,
        ppdata: *mut *mut core::ffi::c_void,
        pwlanopcodevaluetype: Option<*mut WLAN_OPCODE_VALUE_TYPE>,
    ) -> u32 {
        unsafe {
            WlanQueryInterface(
                hclienthandle,
                pinterfaceguid,
                opcode,
                preserved,
                pdwdatasize,
                ppdata,
                pwlanopcodevaluetype,
            )
        }
    }

    unsafe fn connect(
        hclienthandle: windows::Win32::Foundation::HANDLE,
        pinterfaceguid: *const windows_core::GUID,
        pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS,
        preserved: Option<*const core::ffi::c_void>,
    ) -> u32 {
        unsafe {
            WlanConnect(
                hclienthandle,
                pinterfaceguid,
                pconnectionparameters,
                preserved,
            )
        }
    }

    unsafe fn disconnect(
        hclienthandle: windows::Win32::Foundation::HANDLE,
        pinterfaceguid: *const windows_core::GUID,
        preserved: Option<*const core::ffi::c_void>,
    ) -> u32 {
        unsafe { WlanDisconnect(hclienthandle, pinterfaceguid, preserved) }
    }
    unsafe fn enum_interfaces(
        hclienthandle: windows::Win32::Foundation::HANDLE,
        preserved: Option<*const core::ffi::c_void>,
        ppinterfacelist: *mut *mut WLAN_INTERFACE_INFO_LIST,
    ) -> u32 {
        unsafe { WlanEnumInterfaces(hclienthandle, preserved, ppinterfacelist) }
    }
}

/// Implementation of [`NetworkManager`] for Windows OS.
///
/// `Api`: A type that implements the [`WlanApi`] trait.
pub struct WindowsNetworkManager<Api: WlanApi> {
    _marker: std::marker::PhantomData<Api>,
}

impl<A: WlanApi> Default for WindowsNetworkManager<A> {
    /// Creates a new instance of [`WindowsNetworkManager`]`.
    fn default() -> Self {
        Self::new()
    }
}

impl<A: WlanApi> WindowsNetworkManager<A> {
    /// Creates a new instance of [`WindowsNetworkManager`]`.
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Api: WlanApi> NetworkManager for WindowsNetworkManager<Api> {
    fn reconnect(&self) -> bool {
        Self::open_handle()
            .and_then(|handle| Self::get_network_interface(handle).map(|guid| (handle, guid)))
            .and_then(|(handle, guid)| {
                let profile_name = Self::get_current_profile_name(handle, &guid)?;
                Some((handle, guid, profile_name))
            })
            .map(|(handle, guid, profile)| {
                Self::disconnect(handle, &guid);
                (handle, guid, profile)
            })
            .and_then(|(handle, guid, profile)| {
                let success = Self::connect(handle, &guid, &profile);
                unsafe { Api::close_handle(handle, None) };
                success
            })
            .is_some()
    }
}
impl<Api: WlanApi> WindowsNetworkManager<Api> {
    fn open_handle() -> Option<HANDLE> {
        let mut client_handle = HANDLE(std::ptr::null_mut());
        let mut negotiated_version = 0u32;

        let result =
            unsafe { Api::open_handle(2, None, &mut negotiated_version, &mut client_handle) };

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

            unsafe { Api::free_memory(iface_list_ptr as _) };

            guid
        })
    }

    fn get_current_profile_name(handle: HANDLE, iface: &GUID) -> Option<String> {
        let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        let mut data_size: u32 = 0;
        let mut opcode: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(0);

        let result = unsafe {
            Api::query_interface(
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

        unsafe { Api::free_memory(data_ptr as *mut _) };

        Some(profile_name)
    }

    fn disconnect(client_handle: HANDLE, iface: &GUID) -> Option<()> {
        let result = unsafe { Api::disconnect(client_handle, iface, None) };

        if result == ERROR_SUCCESS.0 {
            println!("Disconnected from current Wi-Fi network");
            Some(())
        } else {
            println!("WlanDisconnect failed: {result}");
            None
        }
    }
    fn connect(handle: HANDLE, iface: &GUID, profile: &str) -> Option<()> {
        let utf16: Vec<u16> = profile.encode_utf16().chain(Some(0)).collect();
        let profile_pcwstr = PCWSTR(utf16.as_ptr());

        let result = unsafe {
            Api::connect(
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

        let result = unsafe { Api::enum_interfaces(client_handle, None, &mut iface_list_ptr) };

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
