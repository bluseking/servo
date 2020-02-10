/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::actor::{Actor, ActorMessageStatus, ActorRegistry};
use crate::protocol::JsonPacketStream;
use serde_json::{Map, Value};
use std::net::TcpStream;

pub struct PreferenceActor {
    name: String,
}

impl PreferenceActor {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Actor for PreferenceActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle_message(
        &self,
        _registry: &ActorRegistry,
        msg_type: &str,
        _msg: &Map<String, Value>,
        stream: &mut TcpStream,
    ) -> Result<ActorMessageStatus, ()> {
        Ok(match msg_type {
            "getBoolPref" => {
                let reply = BoolReply {
                    from: self.name(),
                    value: false,
                };
                stream.write_json_packet(&reply);
                ActorMessageStatus::Processed
            },

            "getCharPref" => {
                let reply = CharReply {
                    from: self.name(),
                    value: "".to_owned(),
                };
                stream.write_json_packet(&reply);
                ActorMessageStatus::Processed
            },

            "getIntPref" => {
                let reply = IntReply {
                    from: self.name(),
                    value: 0,
                };
                stream.write_json_packet(&reply);
                ActorMessageStatus::Processed
            },

            _ => ActorMessageStatus::Ignored,
        })
    }
}

#[derive(Serialize)]
struct BoolReply {
    from: String,
    value: bool,
}

#[derive(Serialize)]
struct CharReply {
    from: String,
    value: String,
}

#[derive(Serialize)]
struct IntReply {
    from: String,
    value: i32,
}