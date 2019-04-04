use super::error::{Error, Result};
use crate::areq::AreqIn;
use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, ZpiCmd};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum StartupFromAppStatus {
    /// Restored newtork state
    Restored = 0x00,
    /// New network state
    New = 0x01,
    /// Leave and not Started
    Leave = 0x02,
}
/// ZDO_STARTUP_FROM_APP
#[derive(Serialize, Deserialize, Debug)]
pub struct StartupFromApp {
    /// StartDelay: time before device starts
    pub delay: u16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StartupFromAppRsp {
    pub status: StartupFromAppStatus,
}
impl Sreq for StartupFromApp {
    type Srsp = StartupFromAppRsp;
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x40;
    const MAX_SIZE: usize = 1;
}

/// ZDO_STATE_CHANGE_IND
#[derive(Serialize, Deserialize, Debug)]
pub struct StateChange {
    pub state: u8,
}
impl AreqIn for StateChange {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0xC0;
}

#[derive(Debug)]
pub enum In {
    StateChange(StateChange),
}
impl In {
    pub fn from_cmd(cmd: ZpiCmd) -> Result<Self> {
        match cmd.cmd_id() {
            StateChange::CMD_ID => Ok(In::StateChange(cmd.parse()?)),
            _ => Err(Error::unimplemented(&cmd)),
        }
    }
}
