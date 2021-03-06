//! WIP: Implementation of Web Push ("autopush" as well) in Rust
//!
//! This crate currently provides the connection node functionality for a Web
//! Push server, and is a replacement for the
//! [`autopush`](https://github.com/mozilla-services/autopush) server. The older
//! python handler still manages the HTTP endpoint calls, since those require
//! less overhead to process. Eventually, those functions will be moved to
//! rust as well.
//!
//! # High level overview
//!
//! The entire server is written in an asynchronous fashion using the `futures`
//! crate in Rust. This basically just means that everything is exposed as a
//! future (similar to the concept in other languages) and that's how bits and
//! pieces are chained together.
//!
//! Each connected client maintains a state machine of where it is in the
//! webpush protocol (see `states.dot` at the root of this repository). Note
//! that not all states are implemented yet, this is a work in progress! All I/O
//! is managed by Rust and various state transitions are managed by Rust as
//! well.
//!
//! # Module index
//!
//! There's a number of modules that currently make up the Rust implementation,
//! and one-line summaries of these are:
//!
//! * `queue` - a MPMC queue which is used to send messages to Python and Python
//!   uses to delegate work to worker threads.
//! * `server` - the main bindings for the WebPush server, where the tokio
//!   `Core` is created and managed inside of the Rust thread.
//! * `client` - this is where the state machine for each connected client is
//!   located, managing connections over time and sending out notifications as
//!   they arrive.
//! * `protocol` - a definition of the Web Push protocol messages which are send
//!   over websockets.
//! * `call` - definitions of various calls that can be made into Python, each
//!   of which returning a future of the response.
//!
//! Other modules tend to be miscellaneous implementation details and likely
//! aren't as relevant to the Web Push implementation.
//!
//! Otherwise be sure to check out each module for more documentation!
extern crate base64;
extern crate bytes;
extern crate cadence;
extern crate chan_signal;
extern crate chrono;
extern crate config;
extern crate docopt;
extern crate fernet;
#[macro_use]
extern crate futures;
extern crate futures_backoff;
extern crate hex;
extern crate httparse;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate matches;
extern crate mozsvc_common;
extern crate openssl;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_dynamodb;
extern crate sentry;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_dynamodb;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_mozlog_json;
#[macro_use]
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;
#[macro_use]
extern crate state_machine_future;
extern crate time;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_openssl;
extern crate tokio_service;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate uuid;
extern crate woothee;

#[macro_use]
extern crate error_chain;

#[macro_use]
mod db;
mod client;
pub mod errors;
mod http;
mod logging;
mod protocol;
pub mod server;
pub mod settings;
#[macro_use]
mod util;
