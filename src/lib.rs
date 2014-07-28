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
#![deny(missing_doc)]
#![feature(globs)] // only for tests.

/// A list of MQTT control packet types.
#[deriving(PartialEq, Eq, Clone, Show)]
pub enum PacketKind {
    /// Reserved packet types.
    Reserved,
    /// Connection request.
    Connect,
    /// Connection acknowledgment.
    ConnAck,
    /// Publish messages.
    Publish,
    /// Publish acknowledgment.
    PubAck,
    /// Publish received.
    PubRec,
    /// Publish release.
    PubRel,
    /// Publish complete.
    PubComp,
    /// Subscribe request.
    Subscribe,
    /// Subscribe acknowledgment.
    SubAck,
    /// Unsubscribe request.
    Unsubscribe,
    /// Unsubscribe acknowledgment.
    UnsubAck,
    /// Ping request.
    PingReq,
    /// Ping responce.
    PingResp,
    /// Disconnection.
    Disconnect
}

impl PacketKind {
    /// Returns a `PacketKind` value corresponding to `code`.
    pub fn from_code(code: u8) -> PacketKind {
        match code | 0xf0 {
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

    /// Returns a byte corresponding to `kind`.
    pub fn to_code(kind: PacketKind) -> Option<u8> {
        let code = match kind {
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
            _ => return None
        };
        Some(code)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_code_reserved() {
        assert_eq!(PacketKind::from_code(0x00), Reserved);
        assert_eq!(PacketKind::from_code(0xf0), Reserved);

        // With flag bits
        assert_eq!(PacketKind::from_code(0x01), Reserved);
        assert_eq!(PacketKind::from_code(0xff), Reserved);
    }

    #[test]
    fn to_code_reserved() {
        assert_eq!(PacketKind::to_code(Reserved), None);
    }
}
