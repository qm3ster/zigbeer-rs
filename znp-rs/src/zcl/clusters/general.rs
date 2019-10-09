use super::super::frame::ZclFrame;
use super::error::{Error, Result};
use byteorder::{LittleEndian, ReadBytesExt};

use bytes::Buf;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::convert::TryInto;

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq)]
pub enum VarType {
    Uint8 = 0x20,
    Uint16 = 0x21,
    Uint40 = 0x24,
    Int8 = 0x28,
    Int16 = 0x29,
    Bool = 0x10,
    Octstr = 0x41,
    String = 0x42,
    Struct = 0x4c,
}

use std::io::{self, BufRead, Read};

impl VarType {
    fn read<R: BufRead>(&self, reader: &mut R) -> io::Result<VarTypeVal> {
        let val = match self {
            VarType::Uint8 => VarTypeVal::Uint8(reader.read_u8()?),
            VarType::Uint16 => VarTypeVal::Uint16(reader.read_u16::<LittleEndian>()?),
            VarType::Uint40 => {
                let low = reader.read_u32::<LittleEndian>()?;
                let high = reader.read_u8()?;
                VarTypeVal::Uint40(((high as u64) << 32) + low as u64)
            }
            VarType::Int8 => VarTypeVal::Int8(reader.read_i8()?),
            VarType::Int16 => VarTypeVal::Int16(reader.read_i16::<LittleEndian>()?),
            VarType::Bool => VarTypeVal::Bool(reader.read_u8()? != 0),
            VarType::Octstr => {
                let len = reader.read_u8()?;
                let mut buf = vec![0; len.try_into().unwrap()];
                reader.read_exact(&mut buf)?;
                VarTypeVal::Octstr(buf)
            }
            VarType::String => {
                let len = reader.read_u8()?;
                let mut buf = vec![0; len.try_into().unwrap()];
                reader.read_exact(&mut buf)?;
                VarTypeVal::String(buf)
            }
            VarType::Struct => {
                let len = reader.read_u16::<LittleEndian>()?;
                let mut out = Vec::with_capacity(len.try_into().unwrap());
                println!("will read {}", len);
                for _i in 0..len {
                    let val = match VarTypeVal::parse(reader) {
                        Err(err) => {
                            eprintln!("{}", err);
                            break;
                        }
                        Ok(val) => val,
                    };
                    out.push(val)
                }
                VarTypeVal::Struct(out)
            }
        };
        Ok(val)
    }
}

#[derive(Debug)]
pub enum VarTypeVal {
    Uint8(u8),
    Uint16(u16),
    Uint40(u64),
    Int8(i8),
    Int16(i16),
    Bool(bool),
    Octstr(Vec<u8>),
    String(Vec<u8>),
    Struct(Vec<VarTypeVal>),
}

impl VarTypeVal {
    fn parse<R: BufRead>(reader: &mut R) -> std::result::Result<Self, String> {
        let typ = reader
            .read_u8()
            .map_err(|err| "Unexpected end: can't read next VarType type")?;
        let typ = VarType::from_u8(typ).ok_or_else(|| format!("Unknown VarType: 0x{:x}", typ))?;
        typ.read(reader)
            .map_err(|err| format!("Couldn't read type {:?}", typ))
    }
}

#[derive(Debug)]
pub struct AttrReport {
    attrs: HashMap<u16, VarTypeVal>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq)]
pub enum CmdId {
    AttrReport = 0x0a,
    DiscoverCmdsRecRsp = 0x12,
}

#[derive(Debug)]
pub struct DiscoverCmdsRecRsp {
    discovery_complete: bool,
    cmd_ids: Vec<u8>,
}

#[derive(Debug)]
pub enum In {
    AttrReport(AttrReport),
    DiscoverCmdsRecRsp(DiscoverCmdsRecRsp),
}
impl In {
    pub fn from_cmd(frame: ZclFrame) -> Result<Self> {
        let cmd_id =
            CmdId::from_u8(frame.cmd_id).ok_or_else(|| Error::unknown_cmd(frame.cmd_id))?;
        let mut payload = std::io::Cursor::new(frame.payload);
        match cmd_id {
            CmdId::AttrReport => {
                let mut attrs = HashMap::new();
                let mut cur = payload.reader();
                while cur.get_ref().has_remaining() {
                    let index = match cur.read_u16::<LittleEndian>() {
                        Err(err) => {
                            eprintln!("Unexpected end: can't read next Attribute index");
                            eprintln!("Inner error: {}", err);
                            break;
                        }
                        Ok(index) => index,
                    };
                    let val = match VarTypeVal::parse(&mut cur) {
                        Err(err) => {
                            eprintln!("{}", err);
                            break;
                        }
                        Ok(val) => val,
                    };
                    attrs.insert(index, val);
                }
                Ok(In::AttrReport(AttrReport { attrs }))
            }
            CmdId::DiscoverCmdsRecRsp => {
                let mut cur = (&mut payload).reader();
                let discovery_complete = cur.read_u8().unwrap() != 0;
                let cmd_ids = payload.collect();
                Ok(In::DiscoverCmdsRecRsp(DiscoverCmdsRecRsp {
                    discovery_complete,
                    cmd_ids,
                }))
            }
        }
    }
}
