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
        match code & 0xf0 {
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
mod tests {
    use super::*;

    #[test]
    fn from_code() {
        // Without flag bits
        assert_eq!(PacketKind::from_code(0x10), Connect);
        assert_eq!(PacketKind::from_code(0x20), ConnAck);
        assert_eq!(PacketKind::from_code(0x30), Publish);
        assert_eq!(PacketKind::from_code(0x40), PubAck);
        assert_eq!(PacketKind::from_code(0x50), PubRec);
        assert_eq!(PacketKind::from_code(0x60), PubRel);
        assert_eq!(PacketKind::from_code(0x70), PubComp);
        assert_eq!(PacketKind::from_code(0x80), Subscribe);
        assert_eq!(PacketKind::from_code(0x90), SubAck);
        assert_eq!(PacketKind::from_code(0xa0), Unsubscribe);
        assert_eq!(PacketKind::from_code(0xb0), UnsubAck);
        assert_eq!(PacketKind::from_code(0xc0), PingReq);
        assert_eq!(PacketKind::from_code(0xd0), PingResp);
        assert_eq!(PacketKind::from_code(0xe0), Disconnect);

        // With flag bits
        assert_eq!(PacketKind::from_code(0x3f), Publish);
        assert_eq!(PacketKind::from_code(0x62), PubRel);
        assert_eq!(PacketKind::from_code(0x82), Subscribe);
        assert_eq!(PacketKind::from_code(0xa2), Unsubscribe);
    }

    #[test]
    fn from_code_reserved() {
        // Without flag bits
        assert_eq!(PacketKind::from_code(0x00), Reserved);
        assert_eq!(PacketKind::from_code(0xf0), Reserved);

        // With flag bits
        assert_eq!(PacketKind::from_code(0x01), Reserved);
        assert_eq!(PacketKind::from_code(0xff), Reserved);
    }

    #[test]
    fn to_code() {
        assert_eq!(PacketKind::to_code(Connect), Some(0x10));
        assert_eq!(PacketKind::to_code(ConnAck), Some(0x20));
        assert_eq!(PacketKind::to_code(Publish), Some(0x30));
        assert_eq!(PacketKind::to_code(PubAck), Some(0x40));
        assert_eq!(PacketKind::to_code(PubRec), Some(0x50));
        assert_eq!(PacketKind::to_code(PubRel), Some(0x60));
        assert_eq!(PacketKind::to_code(PubComp), Some(0x70));
        assert_eq!(PacketKind::to_code(Subscribe), Some(0x80));
        assert_eq!(PacketKind::to_code(SubAck), Some(0x90));
        assert_eq!(PacketKind::to_code(Unsubscribe), Some(0xa0));
        assert_eq!(PacketKind::to_code(UnsubAck), Some(0xb0));
        assert_eq!(PacketKind::to_code(PingReq), Some(0xc0));
        assert_eq!(PacketKind::to_code(PingResp), Some(0xd0));
        assert_eq!(PacketKind::to_code(Disconnect), Some(0xe0));
    }

    #[test]
    fn to_code_reserved() {
        assert_eq!(PacketKind::to_code(Reserved), None);
    }
}
