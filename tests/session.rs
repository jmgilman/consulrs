mod common;

use common::{ConsulServer, ConsulServerHelper};
use consulrs::{api::session::requests::CreateSessionRequest, client::Client, session};
use test_env_log::test;

#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: ConsulServer = instance.server();
        let client = server.client();
        let node = server.node().await;

        let uuid = test_create(&client).await;
        test_read(&client, &uuid).await;
        test_list(&client).await;
        test_list_by_node(&client, &node).await;
        test_renew(&client, &uuid).await;
        test_delete(&client, &uuid).await;
    });
}

async fn test_create(client: &impl Client) -> String {
    let res = session::create(client, Some(CreateSessionRequest::builder().ttl("10m"))).await;
    assert!(res.is_ok());

    res.unwrap().response.id.clone()
}

async fn test_delete(client: &impl Client, name: &str) {
    let res = session::delete(client, name, None).await;
    assert!(res.is_ok());
}

async fn test_list(client: &impl Client) {
    let res = session::list(client, None).await;
    assert!(res.is_ok());
}

async fn test_list_by_node(client: &impl Client, node: &str) {
    let res = session::list_by_node(client, node, None).await;
    assert!(res.is_ok());
}

async fn test_read(client: &impl Client, name: &str) {
    let res = session::read(client, name, None).await;
    assert!(res.is_ok());
}

async fn test_renew(client: &impl Client, name: &str) {
    let res = session::renew(client, name, None).await;
    assert!(res.is_ok());
}
