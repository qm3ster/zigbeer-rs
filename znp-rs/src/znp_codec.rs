use bytes::{BufMut, BytesMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::de::DeserializeOwned;
use std::io;
use tokio::codec::{Decoder, Encoder};

const SOF: u8 = 0xFE;

fn xor(buf: &[u8]) -> u8 {
    buf.iter().fold(0x00, |acc, x| acc ^ x)
}

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum Type {
    POLL = 0x00,
    SREQ = 0x20,
    AREQ = 0x40,
    SRSP = 0x60,
}

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum Subsys {
    Reserved = 0x00,
    SYS = 0x01,
    MAC = 0x02,
    NWK = 0x03,
    AF = 0x04,
    ZDO = 0x05,
    SAPI = 0x06,
    UTIL = 0x07,
    DEBUG = 0x08,
    APP = 0x09,
}
#[derive(Debug)]
pub struct ZpiCmd {
    typ: Type,
    subsys: Subsys,
    cmd_id: u8,
    body: BytesMut,
}
impl ZpiCmd {
    pub fn new(typ: Type, subsys: Subsys, cmd_id: u8, body: BytesMut) -> Self {
        ZpiCmd {
            typ,
            subsys,
            cmd_id,
            body,
        }
    }
    pub fn typ(&self) -> Type {
        self.typ
    }
    pub fn subsys(&self) -> Subsys {
        self.subsys
    }
    pub fn cmd_id(&self) -> u8 {
        self.cmd_id
    }
    pub fn parse<T: DeserializeOwned>(&self) -> crate::serde_znp::Result<T> {
        crate::serde_znp::deserialize(&self.body)
    }
}
pub struct ZnpCodec;
impl Decoder for ZnpCodec {
    type Item = ZpiCmd;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let len = buf.len();
        // Minimum frame is:
        // SOF + Length + Cmd0 + Cmd1 + FCS
        // [0xFE, 0x00, 0xXX, 0xXX, 0xXX]
        if len < 5 {
            return Ok(None);
        }
        if buf[0] != SOF {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Desync: SOF byte not 0xFE",
            ));
        }
        let length = buf[1] as usize;
        let frame_len = 5 + length;
        if len < frame_len {
            return Ok(None);
        }
        let mut frame = buf.split_to(frame_len);
        // Skip: SOF
        frame.advance(1);
        // Remaining: Length + Cmd0 + Cmd1 + Body{Length} + FCS
        if xor(&frame) != 0 {
            // XORing including the FCS
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Desync: FCS mismatch",
            ));
        }
        // Drop: FCS
        frame.truncate(frame.len() - 1);
        let cmd0 = frame[1];
        let typ = Type::from_u8(cmd0 & 0xf0)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unknown Type"))?;
        let subsys = Subsys::from_u8(cmd0 & 0xf)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unknown Subsystem"))?;
        let cmd_id = frame[2];
        // Skip: Length + Cmd0 + Cmd1
        frame.advance(3);
        Ok(Some(ZpiCmd {
            typ,
            subsys,
            cmd_id,
            body: frame,
        }))
    }
}
impl Encoder for ZnpCodec {
    type Item = ZpiCmd;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let ZpiCmd {
            typ,
            subsys,
            cmd_id,
            body,
        } = item;
        let length = body.len();
        if length > 250 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "length over 250",
            ));
        }
        buf.reserve(length + 5);
        buf.put(SOF);
        buf.put::<u8>(length as u8);
        let cmd0 = typ as u8 + subsys as u8;
        buf.put(cmd0);
        let cmd1 = cmd_id;
        buf.put(cmd1);
        buf.put(body);
        buf.put(xor(&buf[1..buf.len()]));
        Ok(())
    }
}
