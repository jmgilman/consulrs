use std::{collections::HashMap, str::FromStr};

use derive_builder::Builder;
use http::{HeaderValue, Request};

/// An [Endpoint][rustify::Endpoint] which contains optional [Features] for
/// modifying how its generated request is handled.
///
/// All Consul endpoints should derive this trait through [consulrs_derive].
/// While not every endpoint offers all features, each feature by default is
/// optional which allows an end-user to specify the features they would like
/// enabled for the request. This crate does not perform any checks to ensure
/// features are being used correctly - incorrect usage will result in an API
/// error which will eventually make it back to the end-user.
pub trait FeaturedEndpoint {
    fn features(&self) -> Option<Features>;
}

/// A set of features which can be applied to an endpoint request.
///
/// The following features are supported:
///
///   * [Consistency Modes](https://www.consul.io/api-docs/features/consistency)
///   * [Blocking Queries](https://www.consul.io/api-docs/features/blocking)
///   * [Filtering](https://www.consul.io/api-docs/features/filtering)
///   * [Caching](https://www.consul.io/api-docs/features/caching)
///
/// By default, all features are optional and must be individually configured
/// in order for them to be applied to a request. Note that not all endpoints
/// support all features or combination of features - this crate performs no
/// checks to ensure correct usage and incorrect usage could result in the API
/// returning an error. Refer to the individual endpoint documentation to verify
/// which features it supports.
#[derive(Builder, Default, Debug, Clone)]
#[builder(setter(strip_option, into), default)]
pub struct Features {
    pub blocking: Option<Blocking>,
    pub cached: Option<String>,
    pub filter: Option<String>,
    pub mode: Option<ConsistencyMode>,
}

impl Features {
    /// Mutates a [Request] by adding query parameters and header fields as
    /// determined by the features configured.
    #[instrument(skip(self, request))]
    pub fn process(&self, request: &mut Request<Vec<u8>>) {
        let mut query = HashMap::<String, String>::new();
        let mut keys = Vec::<String>::new();
        info!("Adding features to request");

        // Blocking Queries
        if let Some(b) = &self.blocking {
            if let Some(w) = &b.wait {
                query.insert("wait".into(), w.clone());
            }

            query.insert("index".into(), b.index.to_string());
        }

        // Caching
        if let Some(c) = &self.cached {
            if !c.is_empty() {
                request
                    .headers_mut()
                    .insert("Cache-Control", HeaderValue::from_str(c.as_str()).unwrap());
            }

            keys.push("cached".into());
        }

        // Filtering
        if let Some(f) = &self.filter {
            query.insert("filter".into(), f.clone());
        }

        // Consistency Modes
        if let Some(m) = &self.mode {
            let mode = match m {
                ConsistencyMode::CONSISTENT => "consistent",
                ConsistencyMode::STALE => "stale",
            };

            keys.push(mode.into())
        }

        let mut url = url::Url::parse(request.uri().to_string().as_str()).unwrap();

        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }

        if !keys.is_empty() {
            url.query_pairs_mut()
                .extend_keys_only::<Vec<String>, String>(keys);
        }

        *request.uri_mut() = http::Uri::from_str(url.as_str()).unwrap();

        info!("Final url with features: {}", request.uri());
    }

    /// Returns a default instance of [FeaturesBuilder] for configuring features.
    pub fn builder() -> FeaturesBuilder {
        FeaturesBuilder::default()
    }
}

/// Configuration options for the Blocking Queries feature.
#[derive(Debug, Clone)]
pub struct Blocking {
    pub index: u64,
    pub wait: Option<String>,
}

/// Configuration options for the Consistency Mode feature.
#[derive(Debug, Clone)]
pub enum ConsistencyMode {
    CONSISTENT,
    STALE,
}
