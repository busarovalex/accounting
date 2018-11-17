#![recursion_limit = "128"]

extern crate base64;
extern crate bincode;
extern crate chrono;
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate lettre;
extern crate lettre_email;
#[macro_use]
extern crate log;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate telegram_bot;
extern crate tokio_core;
extern crate uuid;

mod accounting;
pub mod bot;
pub mod cli;
mod config;
mod dates;
mod error;
pub mod log_util;
mod persistence;
mod registry;
mod representation;
pub mod web;
