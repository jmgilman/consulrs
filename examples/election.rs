#[path = "../tests/common.rs"]
mod common;

use common::{ConsulServer, ConsulServerHelper};
use consulrs::{
    api::{
        features::Blocking,
        kv::requests::{ReadKeyRequest, SetKeyRequest},
        ApiResponse, Features,
    },
    client::ConsulClient,
    error::ClientError,
    kv, session,
};
use futures::future::*;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

/// Represents a service in our ficticious example.
struct Node {
    pub client: ConsulClient,
    pub info: NodeInfo,
    pub key: String,
    pub session: Option<String>,
}

/// The data structure stored in the `lead` key. In this case only the node name
/// is being stored so followers can see who the leader is - more advanced usage
/// would put all relevant information here (i.e. the lead's IP address).
#[derive(Debug, Deserialize, Serialize)]
struct NodeInfo {
    pub name: String,
}

impl Node {
    /// Returns a new [Node].
    pub fn new(client: ConsulClient, name: String, key: String) -> Self {
        Node {
            client,
            info: NodeInfo { name },
            key,
            session: None,
        }
    }

    /// Attempts to acquire leadership for this node.
    ///
    /// Leadership is gained by passsing the `acquire` parameter when writing to
    /// the leader key. The write will only be successful if the key has not
    /// already been acquired by another entity. If successful, `true` is
    /// returned and the this node's `info` is serialized and written to the
    /// key's value.
    pub async fn acquire(&self) -> Result<bool, ClientError> {
        Ok(kv::set_json(
            &self.client,
            &self.key,
            &self.info,
            Some(SetKeyRequest::builder().acquire(self.session.as_ref().unwrap())),
        )
        .await?
        .response)
    }

    /// Reads the current leader information from the leader key.
    pub async fn read_leader(&self) -> Result<ApiResponse<NodeInfo>, ClientError> {
        kv::read_json_raw::<NodeInfo, _>(&self.client, &self.key, None).await
    }

    /// If this node is the leader, releases the leader key to allow other nodes
    /// to attempt to acquire it.
    ///
    /// Releasing is done by passing the `release` parameter when writing to the
    /// leader key. The value of the leader key is also cleared out to match the
    /// fact that there is now no elected leader.
    pub async fn release(&self) -> Result<bool, ClientError> {
        Ok(kv::set(
            &self.client,
            &self.key,
            b"",
            Some(SetKeyRequest::builder().release(self.session.as_ref().unwrap())),
        )
        .await?
        .response)
    }

    /// Acquires a new session for this node.
    ///
    /// Sessions can be roughly considered equal to locks. Each node must hold
    /// a unique session which it uses in order to acquire and release locks on
    /// the leader key.
    pub async fn session(&mut self) -> Result<(), ClientError> {
        self.session = Some(session::create(&self.client, None).await?.response.id);
        Ok(())
    }

    /// Watches the leader key for changes or until the timeout is reached.
    ///
    /// Blocking is accomplished by passing a modify index along with the read
    /// request to the leader key. The modify index must match the last
    /// index change, otherwise this request will return immediately. The HTTP
    /// request will hang until a change in the key is detected or the given
    /// timeout is reached.
    pub async fn watch(&self, index: u64, timeout: &str) {
        kv::read(
            &self.client,
            &self.key,
            Some(
                ReadKeyRequest::builder().features(
                    Features::builder()
                        .blocking(Blocking {
                            index: index,
                            wait: Some(timeout.into()),
                        })
                        .build()
                        .unwrap(),
                ),
            ),
        )
        .await
        .unwrap();
    }
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
                // This is the key we will use for handling elections.
                const LEADER_KEY: &str = "leader";

                let server = &server;
                async move {
                    // The `server.client()` method is just a simple wrapper
                    // that returns a `ConsulClient` already configured to talk
                    // to our Docker container.
                    let mut node = Node::new(
                        server.client(),
                        format!("Node {}", i),
                        LEADER_KEY.to_string(),
                    );
                    println!("{}: starting up", node.info.name);

                    // A session is roughly equivalent to a lock. Each node
                    // must have its own lock in order to uniquely hold the
                    // `leader` key.
                    node.session().await.unwrap();
                    println!(
                        "{}: created session {}",
                        node.info.name,
                        node.session.as_ref().unwrap()
                    );

                    // Here is where election happens. The `acquire()` method
                    // will attempt to acquire a lock on the leader key and
                    // return a `bool` reflecting the status of the attempt. A
                    // successful acquire means that this node is now the
                    // leader.
                    let is_lead = node.acquire().await.unwrap();

                    // Consul returns `true` is the lock was successfully
                    // acquired and `false` if the key was already locked.
                    if is_lead {
                        println!("{}: elected the leader", node.info.name);
                    } else {
                        println!("{}: following", node.info.name);
                    }

                    // We can confirm who the leader is by querying the `lead`
                    // key.
                    let res = node.read_leader().await.unwrap();
                    println!(
                        "{}: the current leader is {}",
                        node.info.name, res.response.name
                    );

                    // To simulate a dropped service the leader will now drop
                    // its lock.
                    if is_lead {
                        sleep(Duration::from_secs(2)).await;
                        println!("{}: dropping lead", node.info.name);

                        // Releasing the lock is as simple as writing to the key
                        // and specifying the `release` parameter with the
                        // session.
                        node.release().await.unwrap();
                        println!("{}: dropped!", node.info.name);
                    } else {
                        // All nodes should be watching the `lead` key for
                        // changes in order to determine if a new election is
                        // needed. This includes the leader, but for the sake of
                        // this example, only the followers will watch the key.
                        println!("{} is watching for changes...", node.info.name);

                        // Watching is done through using the blocking feature
                        // of the KV endpoint.
                        let index = res.index.unwrap().parse::<u64>().unwrap();
                        node.watch(index, "5s").await;

                        // In our example we can assume that if we reached this
                        // point the leader has dropped its lock. Real use-cases
                        // would have all of this logic on loop.
                        println!("{}: attempting to become leader", node.info.name);
                        let is_lead = node.acquire().await.unwrap();

                        if is_lead {
                            println!("{}: elected the leader", node.info.name);
                        } else {
                            println!("{}: following", node.info.name);
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        join_all(a).await;
    });
}
