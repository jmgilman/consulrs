mod common;

use common::{ConsulServer, ConsulServerHelper};
use consulrs::{api::check::requests::RegisterCheckRequest, check, client::Client};
use test_env_log::test;

#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: ConsulServer = instance.server();
        let client = server.client();
        let name = "test";

        test_register(&client, name).await;
        test_list(&client).await;
        test_fail(&client, name).await;
        test_pass(&client, name).await;
        test_warn(&client, name).await;
        test_set_status(&client, name, "critical").await;
        test_deregister(&client, name).await;
    });
}

async fn test_deregister(client: &impl Client, name: &str) {
    let res = check::deregister(client, name, None).await;
    assert!(res.is_ok());
}

async fn test_fail(client: &impl Client, name: &str) {
    let res = check::fail(client, name, None).await;
    assert!(res.is_ok());
}

async fn test_list(client: &impl Client) {
    let res = check::list(client, None).await;
    assert!(res.is_ok());
}

async fn test_pass(client: &impl Client, name: &str) {
    let res = check::pass(client, name, None).await;
    assert!(res.is_ok());
}

async fn test_set_status(client: &impl Client, name: &str, status: &str) {
    let res = check::set_status(client, name, status, None).await;
    assert!(res.is_ok());
}

async fn test_register(client: &impl Client, name: &str) {
    let res = check::register(
        client,
        name,
        Some(RegisterCheckRequest::builder().ttl("10m")),
    )
    .await;
    assert!(res.is_ok());
}

async fn test_warn(client: &impl Client, name: &str) {
    let res = check::warn(client, name, None).await;
    assert!(res.is_ok());
}
