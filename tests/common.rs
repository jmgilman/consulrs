use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
pub use dockertest_server::servers::hashi::{ConsulServer, ConsulServerConfig};
use dockertest_server::Test;

pub const PORT: u32 = 9500;
pub const VERSION: &str = "1.9.9";

pub trait ConsulServerHelper {
    fn client(&self) -> ConsulClient;
}

impl ConsulServerHelper for ConsulServer {
    fn client(&self) -> ConsulClient {
        ConsulClient::new(
            ConsulClientSettingsBuilder::default()
                .address(self.external_url())
                .build()
                .unwrap(),
        )
        .unwrap()
    }
}

// Sets up a new test.
#[allow(dead_code)]
pub fn new_test() -> Test {
    let mut test = Test::default();
    let config = ConsulServerConfig::builder()
        .port(PORT)
        .version(VERSION.into())
        .build()
        .unwrap();
    test.register(config);
    test
}
