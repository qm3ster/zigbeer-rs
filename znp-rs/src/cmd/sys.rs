use super::error::{Error, Result};
use crate::areq::{AreqIn, AreqOut};
use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, ZnpCmd};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// SYS_OSAL_START_TIMER
#[derive(Serialize, Deserialize, Debug)]
pub struct StartTimer {
    /// 0-3
    pub timer_id: u8,
    /// ms
    pub timeout: u16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StartTimerRsp {
    pub status: u8,
}
impl Sreq for StartTimer {
    type Srsp = StartTimerRsp;
    const SUBSYS: Subsys = Subsys::SYS;
    const CMD_ID: u8 = 0x0A;
    const MAX_SIZE: usize = 3;
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ResetReason {
    PowerUp = 0x00,
    External = 0x01,
    Watchdog = 0x02,
}
/// SYS_RESET_IND
#[derive(Serialize, Deserialize, Debug)]
pub struct Reset {
    /// Reason for the reset
    pub reason: ResetReason,
    /// Transport protocol revision
    pub transport_rev: u8,
    pub product_id: u8,
    /// Major release number
    pub major_rel: u8,
    /// Minor release number
    pub minor_rel: u8,
    /// Hardware revision number
    pub hw_rev: u8,
}
impl AreqIn for Reset {
    const SUBSYS: Subsys = Subsys::SYS;
    const CMD_ID: u8 = 0x80;
}

/// SYS_OSAL_TIMER_EXPIRED
#[derive(Serialize, Deserialize, Debug)]
pub struct TimerExpired {
    pub timer_id: u8,
}
impl AreqIn for TimerExpired {
    const SUBSYS: Subsys = Subsys::SYS;
    const CMD_ID: u8 = 0x81;
}

/// SYS_OSAL_NV_READ
#[derive(Serialize, Deserialize, Debug)]
pub struct NvRead {
    /// memory item ID
    pub id: u16,
    /// bytes offset from the beginning or the NV value
    pub offset: u8,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NvReadRsp {
    /// Success 0 or Failure 1
    status: u8,
    value: Vec<u8>,
}
impl Sreq for NvRead {
    type Srsp = NvReadRsp;
    const SUBSYS: Subsys = Subsys::SYS;
    const CMD_ID: u8 = 0x08;
    const MAX_SIZE: usize = 0xFA;
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ResetType {
    /// Hardware/Watchdog reset
    Hard = 0x00,
    /// Jump to reset vector
    Soft = 0x01,
}
/// SYS_RESET_REQ
#[derive(Serialize, Deserialize, Debug)]
pub struct ResetReq {
    pub typ: ResetType,
}
impl AreqOut for ResetReq {
    const SUBSYS: Subsys = Subsys::SYS;
    const CMD_ID: u8 = 0x00;
    const MAX_SIZE: usize = 1;
}

#[derive(Debug)]
pub enum In {
    Reset(Reset),
    TimerExpired(TimerExpired),
}
impl In {
    pub fn from_cmd(cmd: ZnpCmd) -> Result<Self> {
        match cmd.cmd_id() {
            Reset::CMD_ID => Ok(In::Reset(cmd.parse()?)),
            TimerExpired::CMD_ID => Ok(In::TimerExpired(cmd.parse()?)),
            _ => Err(Error::unimplemented(&cmd)),
        }
    }
}
