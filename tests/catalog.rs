mod common;

use common::{ConsulServer, ConsulServerHelper, CountingServer};
use consulrs::{
    api::{
        catalog::requests::{DeregisterEntityRequest, RegisterEntityRequest},
        check::common::AgentCheckBuilder,
    },
    catalog,
    client::Client,
};
use test_log::test;

#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: ConsulServer = instance.server();
        let counting: CountingServer = instance.server();
        let client = server.client();
        let node = server.node().await;
        common::setup(&client, &counting).await;

        test_datacenters(&client).await;
        test_gateway(&client, "test").await;
        test_node(&client, &node).await;
        test_nodes(&client).await;
        test_nodes_with_service(&client, "consul").await;
        test_nodes_with_connect_service(&client, "consul").await;
        test_services(&client).await;
        test_register(&client, &node, "test").await;
        test_deregister(&client, &node, "test").await;
    });
}

async fn test_datacenters(client: &impl Client) {
    let res = catalog::datacenters(client, None).await;
    assert!(res.is_ok());
}

async fn test_deregister(client: &impl Client, node: &str, check: &str) {
    let res = catalog::deregister(
        client,
        node,
        Some(DeregisterEntityRequest::builder().check_id(check)),
    )
    .await;
    assert!(res.is_ok());
}

async fn test_gateway(client: &impl Client, gateway: &str) {
    let res = catalog::gateway(client, gateway, None).await;
    assert!(res.is_ok());
}

async fn test_node(client: &impl Client, node: &str) {
    let res = catalog::node(client, node, None).await;
    assert!(res.is_ok());
}

async fn test_nodes(client: &impl Client) {
    let res = catalog::nodes(client, None).await;
    assert!(res.is_ok());
}

async fn test_nodes_with_service(client: &impl Client, service: &str) {
    let res = catalog::nodes_with_service(client, service, None).await;
    assert!(res.is_ok());
}

async fn test_nodes_with_connect_service(client: &impl Client, service: &str) {
    let res = catalog::nodes_with_connect_service(client, service, None).await;
    assert!(res.is_ok());
}

async fn test_register(client: &impl Client, node: &str, address: &str) {
    let res = catalog::register(
        client,
        node,
        address,
        Some(
            RegisterEntityRequest::builder()
                .check(AgentCheckBuilder::default().name("test").build().unwrap()),
        ),
    )
    .await;
    assert!(res.is_ok());
}

async fn test_services(client: &impl Client) {
    let res = catalog::services(client, None).await;
    assert!(res.is_ok());
}
