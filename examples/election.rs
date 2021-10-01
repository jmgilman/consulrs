#[path = "../tests/common.rs"]
mod common;

use common::{ConsulServer, ConsulServerHelper};
use consulrs::{
    api::{
        features::Blocking,
        kv::requests::{ReadKeyRequest, SetKeyRequest},
        Features,
    },
    kv, session,
};
use futures::future::*;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

/// The data structure stored in the `lead` key. In this case only the node name
// is being stored so followers can see who the leader is - more advanced usage
// would put all relevant information here (i.e. the lead's IP address).
#[derive(Debug, Deserialize, Serialize)]
struct Node {
    pub name: String,
}

fn main() {
    // Setup a test environment. The `test.run()` method is responsible for
    // bringing up a Consul Docker container to use in the example.
    let test = common::new_test();
    test.run(|instance| async move {
        // A wrapper around the Docker container to make interacting with it
        // easier.
        let server: ConsulServer = instance.server();

        // Create three futures, each acting as a node to represent multiple
        // services in a micro-service architecture. Each node will attempt to
        // become the leader when it initializes.
        let a = (1..4)
            .map(|i| {
                let server = &server;
                async move {
                    let node = Node {
                        name: format!("Node {}", i),
                    };
                    println!("{}: starting up", node.name);

                    // The `server.client()` method is just a simple wrapper
                    // that returns a `ConsulClient` already configured to talk
                    // to our Docker container.
                    let client = server.client();

                    // A session is roughly equivalent to a lock. Each node
                    // must have its own lock in order to uniquely hold the
                    // `leader` key.
                    let session = session::create(&client, None).await.unwrap().response.id;
                    println!("{}: created session {}", node.name, session);

                    // Here is where election happens. By specifying the
                    // `acquire` parameter with our write request we ask Consul
                    // to attempt to lock this key for our unique session. The
                    // actual content we're writing is the `Node` serialized
                    // into a JSON string.
                    let res = kv::set_json(
                        &client,
                        "lead",
                        &node,
                        Some(SetKeyRequest::builder().acquire(&session)),
                    )
                    .await
                    .unwrap();

                    // Consul returns `true` is the lock was successfully
                    // acquired and `false` if the key was already locked.
                    let is_lead = res.response;
                    if is_lead {
                        println!("{}: elected the leader", node.name);
                    } else {
                        println!("{}: following", node.name);
                    }

                    // We can confirm who the leader is by querying the `lead`
                    // key.
                    let res = kv::read_json::<Node, _>(&client, "lead", None)
                        .await
                        .unwrap();
                    println!(
                        "{}: the current leader is {}",
                        node.name, res.response.value.name
                    );

                    // To simulate a dropped service the leader will now drop
                    // its lock.
                    if is_lead {
                        sleep(Duration::from_secs(2)).await;
                        println!("{}: dropping lead", node.name);

                        // Releasing the lock is as simple as writing to the key
                        // and specifying the `release` parameter with the
                        // session.
                        kv::set(
                            &client,
                            "lead",
                            b"",
                            Some(SetKeyRequest::builder().release(&session)),
                        )
                        .await
                        .unwrap();
                        println!("{}: dropped!", node.name);
                    } else {
                        // All nodes should be watching the `lead` key for
                        // changes in order to determine if a new election is
                        // needed. This includes the leader, but for the sake of
                        // this example, only the followers will watch the key.
                        println!("{} is watching for changes...", node.name);

                        // Watching is done through using the blocking feature
                        // of the KV endpoint. When the key changes the index
                        // value also changes. Below we pass the index we got
                        // from our last read which will cause the HTP request
                        // to hang until either a new index is created (because
                        // something happened to the key) or our timeout of five
                        // seconds is reached.
                        kv::read(
                            &client,
                            "lead",
                            Some(
                                ReadKeyRequest::builder().features(
                                    Features::builder()
                                        .blocking(Blocking {
                                            index: res.index.unwrap().parse::<u64>().unwrap(),
                                            wait: Some("5s".into()),
                                        })
                                        .build()
                                        .unwrap(),
                                ),
                            ),
                        )
                        .await
                        .unwrap();

                        // In our example we can assume that if we reached this
                        // point the leader has dropped its lock. Real use-cases
                        // would have all of this logic on loop.
                        println!("{}: attempting to become leader", node.name);
                        let res = kv::set_json(
                            &client,
                            "lead",
                            &node,
                            Some(SetKeyRequest::builder().acquire(session)),
                        )
                        .await
                        .unwrap();

                        let is_lead = res.response;
                        if is_lead {
                            println!("{}: elected the leader", node.name);
                        } else {
                            println!("{}: following", node.name);
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        join_all(a).await;
    });
}
