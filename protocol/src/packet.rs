use std::collections::{BTreeMap};
use std::{fmt};
use cardano::config::{ProtocolMagic};
use cardano::block;
use cardano::block::{HeaderHash};
use cardano::tx;

use cbor_event::{self, se, de::{self, RawCbor}};

type MessageCode = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct HandlerSpec(u16);
impl HandlerSpec {
    pub fn new(c: u16) -> Self { HandlerSpec(c) }
}
impl fmt::Display for HandlerSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl se::Serialize for HandlerSpec {
    fn serialize<W>(&self, serializer: se::Serializer<W>) -> cbor_event::Result<se::Serializer<W>>
        where W: ::std::io::Write
    {
        serializer.write_array(cbor_event::Len::Len(2))?
            .write_unsigned_integer(0)?
            .write_tag(24)?
            .write_bytes(se::Serializer::new_vec().write_unsigned_integer(self.0 as u64)?.finalize())
    }
}
impl de::Deserialize for HandlerSpec {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(2, "HandlerSpec")?;
        let t = raw.unsigned_integer()?;
        if t != 0 {
            return Err(cbor_event::Error::CustomError(format!("Invalid value, expected 0, received {}", t)));
        }
        let tag = raw.tag()?;
        if tag != 24 {
            return Err(cbor_event::Error::CustomError(format!("Invalid tag, expected 24, received {}", tag)));
        }
        let v = RawCbor::from(&raw.bytes()?).unsigned_integer()? as u16;
        Ok(HandlerSpec(v))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct HandlerSpecs(BTreeMap<MessageCode, HandlerSpec>);
impl HandlerSpecs {
    pub fn default_ins() -> Self {
        let mut bm = BTreeMap::new();
        bm.insert(0x04,  HandlerSpec::new(0x05));
        bm.insert(0x05,  HandlerSpec::new(0x04));
        bm.insert(0x06,  HandlerSpec::new(0x07));
        bm.insert(0x22,  HandlerSpec::new(0x5e));
        bm.insert(0x25,  HandlerSpec::new(0x5e));
        bm.insert(0x2b,  HandlerSpec::new(0x5d));
        bm.insert(0x31,  HandlerSpec::new(0x5c));
        bm.insert(0x37,  HandlerSpec::new(0x62));
        bm.insert(0x3d,  HandlerSpec::new(0x61));
        bm.insert(0x43,  HandlerSpec::new(0x60));
        bm.insert(0x49,  HandlerSpec::new(0x5f));
        bm.insert(0x53,  HandlerSpec::new(0x00));
        bm.insert(0x5c,  HandlerSpec::new(0x31));
        bm.insert(0x5d,  HandlerSpec::new(0x2b));
        bm.insert(0x5e,  HandlerSpec::new(0x25));
        bm.insert(0x5f,  HandlerSpec::new(0x49));
        bm.insert(0x60,  HandlerSpec::new(0x43));
        bm.insert(0x61,  HandlerSpec::new(0x3d));
        bm.insert(0x62,  HandlerSpec::new(0x37));
        HandlerSpecs(bm)
    }
    pub fn default_outs() -> Self {
        let mut bm = BTreeMap::new();
        bm.insert(0x04,  HandlerSpec::new(0x05));
        bm.insert(0x05,  HandlerSpec::new(0x04));
        bm.insert(0x06,  HandlerSpec::new(0x07));
        bm.insert(0x0d,  HandlerSpec::new(0x00));
        bm.insert(0x0e,  HandlerSpec::new(0x00));
        bm.insert(0x25,  HandlerSpec::new(0x5e));
        bm.insert(0x2b,  HandlerSpec::new(0x5d));
        bm.insert(0x31,  HandlerSpec::new(0x5c));
        bm.insert(0x37,  HandlerSpec::new(0x62));
        bm.insert(0x3d,  HandlerSpec::new(0x61));
        bm.insert(0x43,  HandlerSpec::new(0x60));
        bm.insert(0x49,  HandlerSpec::new(0x5f));
        bm.insert(0x53,  HandlerSpec::new(0x00));
        HandlerSpecs(bm)
    }
}
impl se::Serialize for HandlerSpecs {
    fn serialize<W>(&self, serializer: se::Serializer<W>) -> cbor_event::Result<se::Serializer<W>>
        where W: ::std::io::Write
    {
        se::serialize_fixed_map(self.0.iter(), serializer)
    }
}
impl de::Deserialize for HandlerSpecs {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        Ok(HandlerSpecs(raw.deserialize()?))
    }
}
impl fmt::Display for HandlerSpecs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for kv in self.0.iter() {
            write!(f, "  * {} -> {}\n", kv.0, kv.1)?;
        }
        write!(f, "")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Handshake {
    pub protocol_magic: ProtocolMagic,
    pub version: block::Version,
    pub in_handlers:  HandlerSpecs,
    pub out_handlers: HandlerSpecs
}
impl Handshake {
    pub fn new(pm: ProtocolMagic, v: block::Version, ins: HandlerSpecs, outs: HandlerSpecs) -> Self {
        Handshake {
            protocol_magic: pm,
            version: v,
            in_handlers: ins,
            out_handlers: outs
        }
    }
}
impl fmt::Display for Handshake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "protocol magic: {:?}", self.protocol_magic)?;
        writeln!(f, "version: {}", self.version)?;
        writeln!(f, "in handlers:\n{}", self.in_handlers)?;
        writeln!(f, "out handlers:\n{}", self.out_handlers)
    }
}
impl Default for Handshake {
    fn default() -> Self {
        Handshake::new(
            ProtocolMagic::default(),
            block::Version::default(),
            HandlerSpecs::default_ins(),
            HandlerSpecs::default_outs(),
        )
    }
}
impl se::Serialize for Handshake {
    fn serialize<W>(&self, serializer: se::Serializer<W>) -> cbor_event::Result<se::Serializer<W>>
        where W: ::std::io::Write
    {
        serializer.write_array(cbor_event::Len::Len(4))?
            .serialize(&self.protocol_magic)?
            .serialize(&self.version)?
            .serialize(&self.in_handlers)?
            .serialize(&self.out_handlers)
    }
}
impl cbor_event::de::Deserialize for Handshake {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(4, "Handshake")?;
        let pm   = raw.deserialize()?;
        let v    = raw.deserialize()?;
        let ins  = raw.deserialize()?;
        let outs = raw.deserialize()?;

        Ok(Handshake::new(pm, v, ins, outs))
    }
}

pub fn send_handshake(hs: &Handshake) -> Vec<u8> { cbor!(hs).unwrap() }

pub type Message = (u8, Vec<u8>);

pub enum MsgType {
    MsgGetHeaders = 0x4,
    MsgHeaders = 0x5,
    MsgGetBlocks = 0x6,
    MsgSubscribe = 0xd,
    MsgAnnounceTx = 0x25, // == InvOrData key TxMsgContents
}

pub fn send_msg_subscribe(keep_alive: bool) -> Message {
    let value = if keep_alive { 43 } else { 42 };
    let dat = se::Serializer::new_vec().write_unsigned_integer(value).unwrap().finalize();
    (MsgType::MsgSubscribe as u8, dat)
}

pub fn send_msg_getheaders(froms: &[block::HeaderHash], to: &Option<block::HeaderHash>) -> Message {
    let serializer = se::Serializer::new_vec().write_array(cbor_event::Len::Len(2)).unwrap();
    let serializer = se::serialize_indefinite_array(froms.iter(), serializer).unwrap();
    let serializer = match to {
        &None    => serializer.write_array(cbor_event::Len::Len(0)).unwrap(),
        &Some(ref h) => {
            serializer.write_array(cbor_event::Len::Len(1)).unwrap()
                .serialize(h).unwrap()
        }
    };
    let dat = serializer.finalize();
    (MsgType::MsgGetHeaders as u8, dat)
}

pub fn send_msg_getblocks(from: &HeaderHash, to: &HeaderHash) -> Message {
    let dat = se::Serializer::new_vec().write_array(cbor_event::Len::Len(2)).unwrap()
        .serialize(from).unwrap()
        .serialize(to).unwrap()
        .finalize();
    (MsgType::MsgGetBlocks as u8, dat)
}

pub fn send_msg_announcetx(txid: &tx::TxId) -> Message {
    let dat = se::Serializer::new_vec().write_array(cbor_event::Len::Len(2)).unwrap()
        .serialize(&0u8).unwrap() // == Left constructor of InvOrData (i.e. InvMsg)
        .serialize(txid).unwrap()
        .finalize();
    (MsgType::MsgAnnounceTx as u8, dat)
}

#[derive(Debug)]
pub enum BlockHeaderResponse {
    Ok(Vec<block::BlockHeader>),
    Err(String)
}
impl fmt::Display for BlockHeaderResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BlockHeaderResponse::Ok(ref ll) => {
                for i in ll {
                    write!(f, "{}\n", i)?;
                }
            },
            &BlockHeaderResponse::Err(ref s) => {
                write!(f, "Err {}\n", s.to_string())?;
            },
        }
        write!(f, "")
    }
}
impl de::Deserialize for BlockHeaderResponse {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(2, "BlockHeaderResponse")?;
        let sum_type = raw.unsigned_integer()?;
        match sum_type {
            0 => {
                Ok(BlockHeaderResponse::Ok(raw.deserialize()?))
            },
            1 => {
                Ok(BlockHeaderResponse::Err(raw.text()?))
            },
            _ => {
                return Err(cbor_event::Error::CustomError(format!("Invalid BlockHeaderResponse: recieved sumtype of {}", sum_type)));
            }
        }
    }
}

#[derive(Debug)]
pub enum BlockResponse {
    Ok(block::Block)
}
impl de::Deserialize for BlockResponse {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(2, "BlockResponse")?;
        let sum_type = raw.unsigned_integer()?;
        match sum_type {
            0 => {
                Ok(BlockResponse::Ok(raw.deserialize()?))
            },
            _ => {
                return Err(cbor_event::Error::CustomError(format!("Invalid BlockHeaderResponse: recieved sumtype of {}", sum_type)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cbor_event::{de::{RawCbor}};

    const GET_BLOCK_HEADER_BYTES : &'static [u8] = &[
          0x82, 0x00, 0x9f, 0x82, 0x01, 0x85, 0x1a, 0x2d
        , 0x96, 0x4a, 0x09, 0x58, 0x20, 0x9d, 0x63, 0xd4, 0x66, 0x7d, 0x43, 0x26, 0x09, 0x8b, 0x1a, 0xb9
        , 0xa9, 0x61, 0xef, 0x30, 0x35, 0xbc, 0xe2, 0x49, 0x99, 0x07, 0xa0, 0x31, 0x24, 0x95, 0x5f, 0xbd
        , 0x58, 0xaf, 0x3e, 0xb8, 0xdc, 0x84, 0x83, 0x01, 0x58, 0x20, 0x9a, 0x01, 0x44, 0x1c, 0x71, 0x68
        , 0x84, 0xd9, 0xe3, 0x20, 0xc1, 0xdf, 0xd6, 0x1f, 0x4c, 0x6d, 0xd4, 0x17, 0x8c, 0x6d, 0x8c, 0x56
        , 0xdb, 0x50, 0x98, 0x60, 0xd8, 0x79, 0x10, 0x89, 0xaf, 0xb3, 0x58, 0x20, 0xef, 0xe1, 0x25, 0x42
        , 0xac, 0xc4, 0xc7, 0x7e, 0x48, 0x46, 0x7c, 0xb4, 0x99, 0xb3, 0xbb, 0xb4, 0x22, 0xd6, 0x52, 0x74
        , 0x5e, 0x91, 0xf9, 0xc3, 0x49, 0x82, 0x89, 0xc8, 0xa4, 0xda, 0x21, 0x6b, 0x82, 0x03, 0x58, 0x20
        , 0xd3, 0x6a, 0x26, 0x19, 0xa6, 0x72, 0x49, 0x46, 0x04, 0xe1, 0x1b, 0xb4, 0x47, 0xcb, 0xcf, 0x52
        , 0x31, 0xe9, 0xf2, 0xba, 0x25, 0xc2, 0x16, 0x91, 0x77, 0xed, 0xc9, 0x41, 0xbd, 0x50, 0xad, 0x6c
        , 0x58, 0x20, 0xaf, 0xc0, 0xda, 0x64, 0x18, 0x3b, 0xf2, 0x66, 0x4f, 0x3d, 0x4e, 0xec, 0x72, 0x38
        , 0xd5, 0x24, 0xba, 0x60, 0x7f, 0xae, 0xea, 0xb2, 0x4f, 0xc1, 0x00, 0xeb, 0x86, 0x1d, 0xba, 0x69
        , 0x97, 0x1b, 0x58, 0x20, 0x4e, 0x66, 0x28, 0x0c, 0xd9, 0x4d, 0x59, 0x10, 0x72, 0x34, 0x9b, 0xec
        , 0x0a, 0x30, 0x90, 0xa5, 0x3a, 0xa9, 0x45, 0x56, 0x2e, 0xfb, 0x6d, 0x08, 0xd5, 0x6e, 0x53, 0x65
        , 0x4b, 0x0e, 0x40, 0x98, 0x84, 0x82, 0x18, 0x2a, 0x19, 0x1e, 0x84, 0x58, 0x40, 0x26, 0x56, 0x6e
        , 0x86, 0xfc, 0x6b, 0x9b, 0x17, 0x7c, 0x84, 0x80, 0xe2, 0x75, 0xb2, 0xb1, 0x12, 0xb5, 0x73, 0xf6
        , 0xd0, 0x73, 0xf9, 0xde, 0xea, 0x53, 0xb8, 0xd9, 0x9c, 0x4e, 0xd9, 0x76, 0xb3, 0x35, 0xb2, 0xb3
        , 0x84, 0x2f, 0x0e, 0x38, 0x00, 0x01, 0xf0, 0x90, 0xbc, 0x92, 0x3c, 0xaa, 0x96, 0x91, 0xed, 0x91
        , 0x15, 0xe2, 0x86, 0xda, 0x94, 0x21, 0xe2, 0x74, 0x5c, 0x7a, 0xcc, 0x87, 0xf1, 0x81, 0x1a, 0x00
        , 0x0d, 0xf5, 0xdd, 0x82, 0x02, 0x82, 0x84, 0x00, 0x58, 0x40, 0x26, 0x56, 0x6e, 0x86, 0xfc, 0x6b
        , 0x9b, 0x17, 0x7c, 0x84, 0x80, 0xe2, 0x75, 0xb2, 0xb1, 0x12, 0xb5, 0x73, 0xf6, 0xd0, 0x73, 0xf9
        , 0xde, 0xea, 0x53, 0xb8, 0xd9, 0x9c, 0x4e, 0xd9, 0x76, 0xb3, 0x35, 0xb2, 0xb3, 0x84, 0x2f, 0x0e
        , 0x38, 0x00, 0x01, 0xf0, 0x90, 0xbc, 0x92, 0x3c, 0xaa, 0x96, 0x91, 0xed, 0x91, 0x15, 0xe2, 0x86
        , 0xda, 0x94, 0x21, 0xe2, 0x74, 0x5c, 0x7a, 0xcc, 0x87, 0xf1, 0x58, 0x40, 0xf1, 0x4f, 0x71, 0x2d
        , 0xc6, 0x00, 0xd7, 0x93, 0x05, 0x2d, 0x48, 0x42, 0xd5, 0x0c, 0xef, 0xa4, 0xe6, 0x58, 0x84, 0xea
        , 0x6c, 0xf8, 0x37, 0x07, 0x07, 0x9e, 0xb8, 0xce, 0x30, 0x2e, 0xfc, 0x85, 0xda, 0xe9, 0x22, 0xd5
        , 0xeb, 0x38, 0x38, 0xd2, 0xb9, 0x17, 0x84, 0xf0, 0x48, 0x24, 0xd2, 0x67, 0x67, 0xbf, 0xb6, 0x5b
        , 0xd3, 0x6a, 0x36, 0xe7, 0x4f, 0xec, 0x46, 0xd0, 0x9d, 0x98, 0x85, 0x8d, 0x58, 0x40, 0x8a, 0xb4
        , 0x3e, 0x90, 0x4b, 0x06, 0xe7, 0x99, 0xc1, 0x81, 0x7c, 0x5c, 0xed, 0x4f, 0x3a, 0x7b, 0xbe, 0x15
        , 0xcd, 0xbf, 0x42, 0x2d, 0xea, 0x9d, 0x2d, 0x5d, 0xc2, 0xc6, 0x10, 0x5c, 0xe2, 0xf4, 0xd4, 0xc7
        , 0x1e, 0x5d, 0x47, 0x79, 0xf6, 0xc4, 0x4b, 0x77, 0x0a, 0x13, 0x36, 0x36, 0x10, 0x99, 0x49, 0xe1
        , 0xf7, 0x78, 0x6a, 0xcb, 0x5a, 0x73, 0x2b, 0xcd, 0xea, 0x04, 0x70, 0xfe, 0xa4, 0x06, 0x58, 0x40
        , 0xc9, 0xd3, 0x57, 0x01, 0x70, 0xd8, 0xa6, 0xb5, 0x16, 0xe2, 0x32, 0xa5, 0xad, 0x79, 0x32, 0xae
        , 0x0a, 0x2c, 0x4d, 0x48, 0x5b, 0x8a, 0x23, 0xe5, 0x68, 0xab, 0x78, 0x43, 0xb6, 0xea, 0x5c, 0xa8
        , 0x68, 0x75, 0xfa, 0x30, 0xd0, 0x82, 0x19, 0x14, 0x24, 0x8b, 0x61, 0x6b, 0xbe, 0x71, 0x80, 0x65
        , 0xfc, 0x56, 0x55, 0xc5, 0xac, 0xc6, 0x73, 0x94, 0x70, 0xdb, 0xa7, 0xe3, 0x03, 0x86, 0xd5, 0x05
        , 0x84, 0x83, 0x00, 0x01, 0x00, 0x82, 0x6a, 0x63, 0x61, 0x72, 0x64, 0x61, 0x6e, 0x6f, 0x2d, 0x73
        , 0x6c, 0x00, 0xa0, 0x58, 0x20, 0x4b, 0xa9, 0x2a, 0xa3, 0x20, 0xc6, 0x0a, 0xcc, 0x9a, 0xd7, 0xb9
        , 0xa6, 0x4f, 0x2e, 0xda, 0x55, 0xc4, 0xd2, 0xec, 0x28, 0xe6, 0x04, 0xfa, 0xf1, 0x86, 0x70, 0x8b
        , 0x4f, 0x0c, 0x4e, 0x8e, 0xdf, 0xff
    ];

    #[test]
    fn parse_get_block_headers_response() {
        let b = RawCbor::from(GET_BLOCK_HEADER_BYTES).deserialize().unwrap();
        match b {
            BlockHeaderResponse::Ok(ll) => assert!(ll.len() == 1),
            BlockHeaderResponse::Err(error) => panic!("test failed: {}", error)
        }
    }

    const HANDSHAKE_BYTES : &'static [u8] = &[
        0x84, 0x1a, 0x2d, 0x96, 0x4a, 0x09, 0x83, 0x00
      , 0x01, 0x00, 0xb3, 0x04, 0x82, 0x00, 0xd8, 0x18, 0x41, 0x05, 0x05, 0x82, 0x00, 0xd8, 0x18, 0x41
      , 0x04, 0x06, 0x82, 0x00, 0xd8, 0x18, 0x41, 0x07, 0x18, 0x22, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18
      , 0x5e, 0x18, 0x25, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x5e, 0x18, 0x2b, 0x82, 0x00, 0xd8, 0x18
      , 0x42, 0x18, 0x5d, 0x18, 0x31, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x5c, 0x18, 0x37, 0x82, 0x00
      , 0xd8, 0x18, 0x42, 0x18, 0x62, 0x18, 0x3d, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x61, 0x18, 0x43
      , 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x60, 0x18, 0x49, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x5f
      , 0x18, 0x53, 0x82, 0x00, 0xd8, 0x18, 0x41, 0x00, 0x18, 0x5c, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18
      , 0x31, 0x18, 0x5d, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x2b, 0x18, 0x5e, 0x82, 0x00, 0xd8, 0x18
      , 0x42, 0x18, 0x25, 0x18, 0x5f, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x49, 0x18, 0x60, 0x82, 0x00
      , 0xd8, 0x18, 0x42, 0x18, 0x43, 0x18, 0x61, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x3d, 0x18, 0x62
      , 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x37, 0xad, 0x04, 0x82, 0x00, 0xd8, 0x18, 0x41, 0x05, 0x05
      , 0x82, 0x00, 0xd8, 0x18, 0x41, 0x04, 0x06, 0x82, 0x00, 0xd8, 0x18, 0x41, 0x07, 0x0d, 0x82, 0x00
      , 0xd8, 0x18, 0x41, 0x00, 0x0e, 0x82, 0x00, 0xd8, 0x18, 0x41, 0x00, 0x18, 0x25, 0x82, 0x00, 0xd8
      , 0x18, 0x42, 0x18, 0x5e, 0x18, 0x2b, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x5d, 0x18, 0x31, 0x82
      , 0x00, 0xd8, 0x18, 0x42, 0x18, 0x5c, 0x18, 0x37, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x62, 0x18
      , 0x3d, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x61, 0x18, 0x43, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18
      , 0x60, 0x18, 0x49, 0x82, 0x00, 0xd8, 0x18, 0x42, 0x18, 0x5f, 0x18, 0x53, 0x82, 0x00, 0xd8, 0x18
      , 0x41, 0x00
    ];

    #[test]
    fn handshake_decoding() {
        let hs = Handshake::default();

        let hs_ : Handshake = RawCbor::from(HANDSHAKE_BYTES).deserialize().unwrap();
        println!("");
        println!("{}", hs.in_handlers);
        println!("{}", hs_.in_handlers);
        assert_eq!(hs.protocol_magic, hs_.protocol_magic);
        assert_eq!(hs.version, hs_.version);
        assert_eq!(hs.in_handlers, hs_.in_handlers);
        assert_eq!(hs.out_handlers, hs_.out_handlers);
        assert_eq!(hs, hs_);
    }

    #[test]
    fn handshake_encoding() {
        let hs = Handshake::default();

        let vec = cbor!(&hs).unwrap();
        assert_eq!(HANDSHAKE_BYTES, vec.as_slice());
    }
}
