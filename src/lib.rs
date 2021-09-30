//! # project-name
//!
//! <p align="center">
//!     <a href="https://!crates.io/crates/project-name">
//!         <img src="https://!img.shields.io/crates/v/project-name">
//!     </a>
//!     <a href="https://!docs.rs/project-name">
//!         <img src="https://!img.shields.io/docsrs/project-name" />
//!     </a>
//!     <a href="https://!github.com/jmgilman/project-name/actions/workflows/ci.yml">
//!         <img src="https://!github.com/jmgilman/project-name/actions/workflows/ci.yml/badge.svg"/>
//!     </a>
//! </p>
//!
//! > A short summary of the project.
//!
//! A longer summary of the project.
//!
//! ## Installation
//!
//! Add `project-name` as a dependency to your cargo.toml:
//! ```ignore
//! [dependencies]
//! project-name = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! Instructions and examples for using the crate.
//!
//! ## Testing
//!
//! Run tests with `cargo test`.

#[macro_use]
extern crate tracing;

pub mod api;
pub mod catalog;
pub mod client;
pub mod error;
pub mod kv;
