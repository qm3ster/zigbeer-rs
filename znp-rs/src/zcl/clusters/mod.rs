pub mod error;
pub mod general;

pub mod on_off;

use super::frame::{FrameType, ZclFrame};
use crate::cmd::af::IncomingMsg;
use bytes::{BufMut, BytesMut};
use error::{Error, Result};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::de::DeserializeOwned;
use std::io;

#[repr(u16)]
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq)]
pub enum ClusterId {
    GenBasic = 0x0000,
    GenOnOff = 0x0006,
}
#[derive(Debug)]
pub enum In {
    GenOnOff(on_off::In),
    General(general::In),
}
impl In {
    pub fn from_incoming(msg: &IncomingMsg) -> Result<Self> {
        use ClusterId::*;
        let cluster =
            ClusterId::from_u16(msg.cluster).ok_or_else(|| Error::unknown_cluster(msg.cluster))?;

        use bytes::buf::IntoBuf;
        let frame = ZclFrame::parse(msg.data.clone().into_buf());
        match frame.frame_control.frame_type {
            FrameType::General => Ok(In::General(general::In::from_cmd(frame)?)),
            FrameType::Cluster => match cluster {
                GenOnOff => Ok(In::GenOnOff(on_off::In::from_cmd(frame)?)),
                cluster => Err(Error::unimplemented_cluster(cluster)),
            },
        }
    }
}
