use super::error::{Error, Result};
use crate::areq::AreqIn;
use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, ZpiCmd};
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub enum In {
    TimerExpired(TimerExpired),
}
impl In {
    pub fn from_cmd(cmd: ZpiCmd) -> Result<Self> {
        match cmd.cmd_id() {
            TimerExpired::CMD_ID => Ok(In::TimerExpired(cmd.parse()?)),
            _ => Err(Error::unimplemented(&cmd)),
        }
    }
}
