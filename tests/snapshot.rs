mod common;

use common::{ConsulServer, ConsulServerHelper};
use consulrs::{client::Client, snapshot};
use test_env_log::test;

#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: ConsulServer = instance.server();
        let client = server.client();

        let snapshot = test_backup(&client).await;
        test_restore(&client, &snapshot).await;
    });
}

async fn test_backup(client: &impl Client) -> Vec<u8> {
    let res = snapshot::backup(client, None).await;
    assert!(res.is_ok());

    res.unwrap().response
}

async fn test_restore(client: &impl Client, snapshot: &[u8]) {
    let res = snapshot::restore(client, snapshot, None).await;
    assert!(res.is_ok());
}
