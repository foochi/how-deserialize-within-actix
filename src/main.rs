#[macro_use]
extern crate log;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod events;

use actix::*;
use serde::de;
use serde::de::{Deserialize, Deserializer};

use events::EventManager;

#[derive(Debug, Message, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    Error(Error),
    Event(Event),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    message: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,
    content: String,
}

fn main() {
    env_logger::init();

    let mut client = EventManager::<Data>new();

    client.capture(|data| debug!("event: {:?}", data));
    client.run();
}
