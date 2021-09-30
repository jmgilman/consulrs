mod common;

use common::{ConsulServer, ConsulServerHelper};
use consulrs::{client::Client, service};
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
        test_read(&client, name).await;
        test_health(&client, name).await;
        test_maintenance(&client, name).await;
        test_deregister(&client, name).await;
    });
}

async fn test_deregister(client: &impl Client, name: &str) {
    let res = service::deregister(client, name, None).await;
    assert!(res.is_ok());
}

async fn test_health(client: &impl Client, name: &str) {
    let res = service::health(client, name, None).await;
    assert!(res.is_ok());
}

async fn test_list(client: &impl Client) {
    let res = service::list(client, None).await;
    assert!(res.is_ok());
}

async fn test_maintenance(client: &impl Client, name: &str) {
    let res = service::maintenance(client, name, true, None).await;
    assert!(res.is_ok());
}

async fn test_register(client: &impl Client, name: &str) {
    let res = service::register(client, name, None).await;
    assert!(res.is_ok());
}

async fn test_read(client: &impl Client, name: &str) {
    let res = service::read(client, name, None).await;
    assert!(res.is_ok());
}
