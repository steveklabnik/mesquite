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
#![allow(dead_code)]

static CONNECT:     u8 = 0x10;
static CONNACK:     u8 = 0x20;
static PUBLISH:     u8 = 0x30;
static PUBACK:      u8 = 0x40;
static PUBREC:      u8 = 0x50;
static PUBREL:      u8 = 0x60;
static PUBCOMP:     u8 = 0x70;
static SUBSCRIBE:   u8 = 0x80;
static SUBACK:      u8 = 0x90;
static UNSUBSCRIBE: u8 = 0xa0;
static UNSUBACK:    u8 = 0xb0;
static PINGREQ:     u8 = 0xc0;
static PINGRESP:    u8 = 0xd0;
static DISCONNECT:  u8 = 0xe0;
