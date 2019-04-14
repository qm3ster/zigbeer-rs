use super::error::{Error, Result};
use super::types::ShortAddr;
use crate::areq::{AreqIn, AreqOut};
use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, ZnpCmd};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// AF_REGISTER
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Register {
    pub ep: u8,
    /// AppProfId
    pub app_prof: u16,
    /// AppDeviceId
    pub dev_type: u16,
    pub dev_ver: u8,
    pub latency_req: u8,
    pub in_clusters: Vec<u16>,
    pub out_clusters: Vec<u16>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRsp {
    /// Success 0 or b8 Already Exists
    pub status: u8,
}
impl Sreq for Register {
    type Srsp = RegisterRsp;
    const SUBSYS: Subsys = Subsys::AF;
    const CMD_ID: u8 = 0x00;
    const MAX_SIZE: usize = 0x49;
}

/// AF_INCOMING_MSG
///
/// This callback message is in response to incoming data to any of the registered endpoints on this device.
#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMsg {
    pub group: u16,
    pub cluster: u16,
    /// Source network address
    pub addr: ShortAddr,
    pub src_ep: u8,
    pub dest_ep: u8,
    pub was_broadcast: bool,
    pub link_quality: u8,
    pub security_used: bool,
    pub timestamp: u32,
    pub tr_seq: u8,
    // #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
impl AreqIn for IncomingMsg {
    const SUBSYS: Subsys = Subsys::AF;
    const CMD_ID: u8 = 0x81;
}

#[derive(Debug)]
pub enum In {
    IncomingMsg(IncomingMsg),
}
impl In {
    pub fn from_cmd(cmd: ZnpCmd) -> Result<Self> {
        match cmd.cmd_id() {
            IncomingMsg::CMD_ID => Ok(In::IncomingMsg(cmd.parse()?)),
            _ => Err(Error::unimplemented(&cmd)),
        }
    }
}
