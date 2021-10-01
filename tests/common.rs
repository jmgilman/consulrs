use async_trait::async_trait;
use consulrs::{
    api::{check::common::AgentServiceCheckBuilder, service::requests::RegisterServiceRequest},
    catalog,
    client::{Client, ConsulClient, ConsulClientSettingsBuilder},
    service,
};
pub use dockertest_server::servers::hashi::{
    counting::{CountingServer, CountingServerConfig},
    ConsulServer, ConsulServerConfig,
};
use dockertest_server::Test;

pub const CHECK_NAME: &str = "health";
pub const CONSUL_PORT: u32 = 9500;
pub const COUNTING_PORT: u32 = 9100;
pub const SERVICE_NAME: &str = "counting";
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

#[derive(Debug)]
pub struct TestService {
    pub name: String,
    pub check: String,
}

// Sets up a new test.
#[allow(dead_code)]
pub fn new_test() -> Test {
    let mut test = Test::default();
    let consul_config = ConsulServerConfig::builder()
        .port(CONSUL_PORT)
        .version(VERSION.into())
        .build()
        .unwrap();
    let counting_config = CountingServerConfig::builder()
        .port(COUNTING_PORT)
        .build()
        .unwrap();
    test.register(consul_config);
    test.register(counting_config);
    test
}

#[allow(dead_code)]
pub async fn setup(client: &impl Client, counting: &CountingServer) -> TestService {
    // Setup test service
    let address = counting.internal_address();
    let port = counting.internal_port;
    let url = counting.internal_url();
    println! {"URL:  {}", url};
    service::register(
        client,
        SERVICE_NAME,
        Some(
            RegisterServiceRequest::builder()
                .address(address)
                .port(port)
                .check(
                    AgentServiceCheckBuilder::default()
                        .name(CHECK_NAME)
                        .interval("1s")
                        .http(url)
                        .status("passing")
                        .build()
                        .unwrap(),
                ),
        ),
    )
    .await
    .unwrap();

    TestService {
        name: SERVICE_NAME.into(),
        check: CHECK_NAME.into(),
    }
}
