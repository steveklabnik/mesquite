// Copyright (c) 2014 Masanori Ogino
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Mesquite: An Implementation of the MQTT Protocol

#![experimental]
#![crate_name = "mesquite"]
#![license = "MIT/ASL2"]
#![feature(globs)] // only for tests.

pub static PROTOCOL_NAME: &'static [u8] = b"MQTT";
pub static PROTOCOL_LEVEL: u8 = 4;

pub enum PacketType {
    Reserved,
    Connect,
    ConnAck,
    Publish,
    PubAck,
    PubRec,
    PubRel,
    PubComp,
    Subscribe,
    SubAck,
    Unsubscribe,
    UnsubAck,
    PingReq,
    PingResp,
    Disconnect,
}

impl PacketType {
    pub fn to_byte(&self) -> u8 {
        match *self {
            Connect => 0x10,
            ConnAck => 0x20,
            Publish => 0x30,
            PubAck => 0x40,
            PubRec => 0x50,
            PubRel => 0x60,
            PubComp => 0x70,
            Subscribe => 0x80,
            SubAck => 0x90,
            Unsubscribe => 0xa0,
            UnsubAck => 0xb0,
            PingReq => 0xc0,
            PingResp => 0xd0,
            Disconnect => 0xe0,
            _ => panic!("Converting from a reserved packet type.")
        }
    }

    pub fn from_byte(header: u8) -> PacketType {
        match header & 0xf0 {
            0x10 => Connect,
            0x20 => ConnAck,
            0x30 => Publish,
            0x40 => PubAck,
            0x50 => PubRec,
            0x60 => PubRel,
            0x70 => PubComp,
            0x80 => Subscribe,
            0x90 => SubAck,
            0xa0 => Unsubscribe,
            0xb0 => UnsubAck,
            0xc0 => PingReq,
            0xd0 => PingResp,
            0xe0 => Disconnect,
            _ => Reserved
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
