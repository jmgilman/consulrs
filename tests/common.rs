use async_trait::async_trait;
use consulrs::{
    catalog,
    client::{ConsulClient, ConsulClientSettingsBuilder},
};
pub use dockertest_server::servers::hashi::{ConsulServer, ConsulServerConfig};
use dockertest_server::Test;

pub const PORT: u32 = 9500;
pub const VERSION: &str = "1.9.9";

#[async_trait]
pub trait ConsulServerHelper {
    /// Returns a [ConsulClient] configured to connect to the [ConsulServer].
    fn client(&self) -> ConsulClient;

    /// Returns the node ID for the default node of the [ConsulServer].
    async fn node(&self) -> String;
}

#[async_trait]
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
    async fn node(&self) -> String {
        let res = catalog::nodes(&self.client(), None).await;
        res.unwrap().response.pop().unwrap().node
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
