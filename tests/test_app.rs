#[cfg(test)]
use mockall::{mock, predicate::*};

use internet_reloader::app::{NetworkApp, NetworkStatus};
use internet_reloader::internet_connectivity::InternetConnectivity;
use internet_reloader::network_manager::NetworkManager;

mock! {
    NetworkManager {}
    impl NetworkManager for NetworkManager {
        fn reconnect(&self) -> bool;
    }
}

mock! {
    InternetConnectivity {}
    impl InternetConnectivity for InternetConnectivity {
        fn is_connected_to_network(&self) -> bool;
        fn is_connected_to_internet(&self) -> bool;
    }
}

struct PollCase {
    network_ok: bool,
    internet_ok: bool,
    reconnect_returns: Option<bool>,
    expected_status: NetworkStatus,
}

macro_rules! poll_tests {
    ($($name:ident: $case:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let case = $case;

                let mut checker = MockInternetConnectivity::new();
                let mut manager = MockNetworkManager::new();

                checker
                    .expect_is_connected_to_network()
                    .return_const(case.network_ok);

                checker
                    .expect_is_connected_to_internet()
                    .return_const(case.internet_ok);

                match case.reconnect_returns {
                    Some(_) => {
                        manager
                            .expect_reconnect()
                            .return_const(case.reconnect_returns.unwrap());
                    }
                    None => {
                        manager.expect_reconnect().times(0);
                    }
                }

                let app = NetworkApp::new(checker, manager);
                assert_eq!(app.poll(), case.expected_status);
            }
        )*
    };
}

poll_tests! {
    fully_connected: PollCase {
        network_ok: true,
        internet_ok: true,
        reconnect_returns: None,
        expected_status: NetworkStatus::Connected,
    },
    disconnected: PollCase {
        network_ok: false,
        internet_ok: false,
        reconnect_returns: None,
        expected_status: NetworkStatus::Disconnected,
    },

    network_only_reconnect_success: PollCase {
        network_ok: true,
        internet_ok: false,
        reconnect_returns: Some(true),
        expected_status: NetworkStatus::Connected,
    },
    network_only_reconnect_fail: PollCase {
        network_ok: true,
        internet_ok: false,
        reconnect_returns: Some(false),
        expected_status: NetworkStatus::NetworkOnly,
    },
}
