use super::super::frame::ZclFrame;
use super::error::{Error, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq)]
pub enum VarType {
    Uint8 = 0x00,
    Uint16 = 0x01,
    Bool = 0x10,
    // String = 0x42,
}

use std::io::{self, BufRead, Read};

impl VarType {
    fn read<R: BufRead>(&self, reader: &mut R) -> io::Result<VarTypeVal> {
        let val = match self {
            VarType::Uint8 => VarTypeVal::Uint8(reader.read_u8()?),
            VarType::Uint16 => VarTypeVal::Uint16(reader.read_u16::<LittleEndian>()?),
            VarType::Bool => VarTypeVal::Bool(reader.read_u8()? != 0),
            // VarType::String => {
            //     let len = reader.read_u8()?;
            //     let (a,b)=reader.split(len)?;
            // },
        };
        Ok(val)
    }
}

#[derive(Debug)]
pub enum VarTypeVal {
    Uint8(u8),
    Uint16(u16),
    Bool(bool),
    String(String),
}

#[derive(Debug)]
pub struct AttrReport {
    attrs: HashMap<u16, VarTypeVal>,
}

#[derive(Debug)]
pub enum In {
    AttrReport(AttrReport),
}
impl In {
    pub fn from_cmd(cmd: ZclFrame) -> Result<Self> {
        match cmd.cmd_id {
            0x0a => {
                let mut attrs = HashMap::new();
                let end = cmd.payload.len() as u64 - 1;
                let mut cur = std::io::Cursor::new(cmd.payload);
                while cur.position() < end {
                    let index = cur.read_u16::<LittleEndian>().unwrap();
                    let typ = cur.read_u8().unwrap();
                    let typ = VarType::from_u8(typ).unwrap();
                    let val = typ.read(&mut cur).unwrap();
                    attrs.insert(index, val);
                }
                Ok(In::AttrReport(AttrReport { attrs }))
            }
            _ => {
                println!("{:x?}", cmd);
                Err(Error::unimplemented_cmd(cmd.cmd_id))
            }
        }
    }
}
