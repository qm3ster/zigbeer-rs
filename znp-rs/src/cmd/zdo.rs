use super::error::{Error, Result};
use super::types::{IEEEAddr, ShortAddr};
use crate::areq::AreqIn;
use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, ZnpCmd};
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

/// ZDO_END_DEVICE_ANNCE_IND
#[derive(Serialize, Deserialize, Debug)]
pub struct EndDevAnnce {
    pub src_addr: ShortAddr,
    pub nwk_addr: ShortAddr,
    pub ieee_addr: IEEEAddr,
    pub capabilities: u8,
}
impl AreqIn for EndDevAnnce {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0xC1;
}

/// ZDO_SRC_RTG_IND
#[derive(Serialize, Deserialize, Debug)]
pub struct SourceRoute {
    pub dst_addr: ShortAddr,
    pub relay_list: Vec<ShortAddr>,
}
impl AreqIn for SourceRoute {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0xC4;
}

/// ZDO_LEAVE_IND
///
/// A device is leaving the network
#[derive(Serialize, Deserialize, Debug)]
pub struct Leaving {
    /// Source network address
    pub addr: ShortAddr,
    /// Source IEEE address
    pub ieee_addr: IEEEAddr,
    /// true: request, false: indication
    pub request: bool,
    /// true: remove children
    pub children: bool,
    /// true: rejoin
    pub rejoin: bool,
}
impl AreqIn for Leaving {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0xC9;
}

/// ZDO_TC_DEV_IND
///
/// ZDO callback for Trust Center Device Indication
#[derive(Serialize, Deserialize, Debug)]
pub struct TrustCntDev {
    /// Source network address
    pub addr: ShortAddr,
    /// Source IEEE address
    pub ieee_addr: IEEEAddr,
    /// Parent network address
    pub parent_addr: ShortAddr,
}
impl AreqIn for TrustCntDev {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0xCA;
}

#[derive(Debug)]
pub enum In {
    StateChange(StateChange),
    SourceRoute(SourceRoute),
    EndDevAnnce(EndDevAnnce),
    Leaving(Leaving),
    TrustCntDev(TrustCntDev),
}
impl In {
    pub fn from_cmd(cmd: ZnpCmd) -> Result<Self> {
        match cmd.cmd_id() {
            StateChange::CMD_ID => Ok(In::StateChange(cmd.parse()?)),
            SourceRoute::CMD_ID => Ok(In::SourceRoute(cmd.parse()?)),
            EndDevAnnce::CMD_ID => Ok(In::EndDevAnnce(cmd.parse()?)),
            Leaving::CMD_ID => Ok(In::Leaving(cmd.parse()?)),
            TrustCntDev::CMD_ID => Ok(In::TrustCntDev(cmd.parse()?)),
            _ => Err(Error::unimplemented(&cmd)),
        }
    }
}
