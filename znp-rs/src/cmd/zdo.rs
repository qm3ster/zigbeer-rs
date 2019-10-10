use super::error::{Error, Result};
use super::types::{Endpoint, IEEEAddr, ShortAddr};
use crate::areq::AreqIn;
use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, ZnpCmd};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// ZDO_NODE_DESC_REQ
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeDescReq {
    /// Address to respond to
    pub dest_addr: ShortAddr,
    /// NWKAddrOfInterest - Specifies NWK address of the destination device being queried
    pub query_addr: ShortAddr,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeDescReqRsp {
    /// Success 0 or Failure 1
    pub status: u8,
}
impl Sreq for NodeDescReq {
    type Srsp = NodeDescReqRsp;
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x02;
    const MAX_SIZE: usize = 0x04;
}

// #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
// #[repr(u8)]
// pub enum LogicalType {
//     Coordinator = 0x00,
//     Router = 0x01,
//     EndDevice = 0x02,
// }

/// ZDO_NODE_DESC_RSP
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeDescRsp {
    pub src_addr: ShortAddr,
    /// Success 0 or Failure 1
    pub status: u8,
    pub query_addr: ShortAddr,
    /// LogicalType/ComplexDescriptorAvailable/UserDescriptorAvailable
    pub field1: u8,
    /// APSFlags/FrequencyBand
    pub field2: u8,
    pub mac_capabilities: u8,
    pub manuf_code: u16,
    pub max_buffer_size: u8,
    pub max_in_transfer_size: u16,
    pub server_mask: u16,
    pub max_out_transfer_size: u16,
    pub descriptor_capabilities: u8,
}
impl AreqIn for NodeDescRsp {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x82;
}

/// ZDO_POWER_DESC_REQ
#[derive(Serialize, Deserialize, Debug)]
pub struct PowerDescReq {
    /// Address to respond to
    pub dest_addr: ShortAddr,
    /// NWKAddrOfInterest - Specifies NWK address of the destination device being queried
    pub query_addr: ShortAddr,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PowerDescReqRsp {
    /// Success 0 or Failure 1
    pub status: u8,
}
impl Sreq for PowerDescReq {
    type Srsp = PowerDescReqRsp;
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x03;
    const MAX_SIZE: usize = 0x04;
}

/// ZDO_POWER_DESC_RSP
#[derive(Serialize, Deserialize, Debug)]
pub struct PowerDescRsp {
    pub src_addr: ShortAddr,
    /// Success 0 or Failure 1
    pub status: u8,
    pub query_addr: ShortAddr,
    /// CurrentPowerMode/AvailablePowerSources
    pub field1: u8,
    /// CurrentPowerSource/CurrentPowerSourceLevel
    pub field2: u8,
}
impl AreqIn for PowerDescRsp {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x83;
}

/// ZDO_SIMPLE_DESC_REQ
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleDescReq {
    /// Address to respond to
    pub dest_addr: ShortAddr,
    /// NWKAddrOfInterest - Specifies NWK address of the destination device being queried
    pub query_addr: ShortAddr,
    /// Specifies the application endpoint the data is from
    pub endpoint: Endpoint,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleDescReqRsp {
    /// Success 0 or Failure 1
    pub status: u8,
}
impl Sreq for SimpleDescReq {
    type Srsp = SimpleDescReqRsp;
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x04;
    const MAX_SIZE: usize = 0x04;
}

/// ZDO_SIMPLE_DESC_RSP
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleDescRsp {
    pub src_addr: ShortAddr,
    /// Success 0 or Failure 1
    pub status: u8,
    pub query_addr: ShortAddr,
    /// Specifies the length of the simple descriptor
    pub len: u8,
    pub endpoint: Endpoint,
    /// Profile Id for endpoint
    pub profile_id: u16,
    /// Device Description Id for endpoint
    pub device_id: u16,
    /// 0 = Version 1.00, 0x01-0x0F = Reserved
    pub device_version: u8,
    pub input_clusters: Vec<u16>,
    pub output_clusters: Vec<u16>,
}
impl AreqIn for SimpleDescRsp {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x84;
}

/// ZDO_ACTIVE_EP_REQ
#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveEpReq {
    /// Address to respond to
    pub dest_addr: ShortAddr,
    /// NWKAddrOfInterest - Specifies NWK address of the destination device being queried
    pub query_addr: ShortAddr,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveEpReqRsp {
    /// Success 0 or Failure 1
    pub status: u8,
}
impl Sreq for ActiveEpReq {
    type Srsp = ActiveEpReqRsp;
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x05;
    const MAX_SIZE: usize = 0x04;
}

/// ZDO_ACTIVE_EP_RSP
#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveEpRsp {
    pub src_addr: ShortAddr,
    /// Success 0 or Failure 1
    pub status: u8,
    pub query_addr: ShortAddr,
    pub active_eps: Vec<u8>,
}
impl AreqIn for ActiveEpRsp {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x85;
}

/// ZDO_COMPLEX_DESC_REQ
#[derive(Serialize, Deserialize, Debug)]
pub struct ComplexDescReq {
    /// Address to respond to
    pub dest_addr: ShortAddr,
    /// NWKAddrOfInterest - Specifies NWK address of the destination device being queried
    pub query_addr: ShortAddr,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ComplexDescReqRsp {
    /// Success 0 or Failure 1
    pub status: u8,
}
impl Sreq for ComplexDescReq {
    type Srsp = ComplexDescReqRsp;
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x07;
    const MAX_SIZE: usize = 0x04;
}

/// ZDO_COMPLEX_DESC_RSP
#[derive(Serialize, Deserialize, Debug)]
pub struct ComplexDescRsp {
    pub src_addr: ShortAddr,
    /// Success 0 or Failure 1
    pub status: u8,
    pub query_addr: ShortAddr,
    /// Array of bytes contains the complex descriptor
    pub complex_descriptor: Vec<u8>,
}
impl AreqIn for ComplexDescRsp {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x87;
}

///ZDO_MGMT_PERMIT_JOIN_REQ
#[derive(Serialize, Deserialize, Debug)]
pub struct MgmtPermitJoinReq {
    /// Destination address type: 0x02 – Address 16 bit, 0xFF – Broadcast
    pub addr_mode: u8,
    // TODO: Can be Broadcast
    pub dest_addr: ShortAddr,
    /// seconds, 0xff = forever
    pub duration: u8,
    pub tc_significance: u8,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MgmtPermitJoinReqRsp {
    /// Success 0 or Failure 1
    pub status: u8,
}
impl Sreq for MgmtPermitJoinReq {
    type Srsp = MgmtPermitJoinReqRsp;
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0x36;
    const MAX_SIZE: usize = 0x05;
}

/// ZDO_MGMT_PERMIT_JOIN_RSP
#[derive(Serialize, Deserialize, Debug)]
pub struct MgmtPermitJoinRsp {
    pub src_addr: ShortAddr,
    /// Success 0 or Failure 1
    pub status: u8,
}
impl AreqIn for MgmtPermitJoinRsp {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0xB6;
}

/// ZDO_MGMT_PERMIT_JOIN_IND
#[derive(Serialize, Deserialize, Debug)]
pub struct MgmtPermitJoinInd {
    /// seconds
    pub duration: u8,
}
impl AreqIn for MgmtPermitJoinInd {
    const SUBSYS: Subsys = Subsys::ZDO;
    const CMD_ID: u8 = 0xCB;
}

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
    pub capabilities: u8, // TODO: Decode bitflags
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
    MgmtPermitJoinRsp(MgmtPermitJoinRsp),
    MgmtPermitJoinInd(MgmtPermitJoinInd),
    NodeDescRsp(NodeDescRsp),
    PowerDescRsp(PowerDescRsp),
    SimpleDescRsp(SimpleDescRsp),
    ActiveEpRsp(ActiveEpRsp),
    ComplexDescRsp(ComplexDescRsp),
    StateChange(StateChange),
    SourceRoute(SourceRoute),
    EndDevAnnce(EndDevAnnce),
    Leaving(Leaving),
    TrustCntDev(TrustCntDev),
}
impl In {
    pub fn from_cmd(cmd: ZnpCmd) -> Result<Self> {
        match cmd.cmd_id() {
            MgmtPermitJoinRsp::CMD_ID => Ok(In::MgmtPermitJoinRsp(cmd.parse()?)),
            MgmtPermitJoinInd::CMD_ID => Ok(In::MgmtPermitJoinInd(cmd.parse()?)),
            NodeDescRsp::CMD_ID => Ok(In::NodeDescRsp(cmd.parse()?)),
            PowerDescRsp::CMD_ID => Ok(In::PowerDescRsp(cmd.parse()?)),
            SimpleDescRsp::CMD_ID => Ok(In::SimpleDescRsp(cmd.parse()?)),
            ActiveEpRsp::CMD_ID => Ok(In::ActiveEpRsp(cmd.parse()?)),
            ComplexDescRsp::CMD_ID => Ok(In::ComplexDescRsp(cmd.parse()?)),
            StateChange::CMD_ID => Ok(In::StateChange(cmd.parse()?)),
            SourceRoute::CMD_ID => Ok(In::SourceRoute(cmd.parse()?)),
            EndDevAnnce::CMD_ID => Ok(In::EndDevAnnce(cmd.parse()?)),
            Leaving::CMD_ID => Ok(In::Leaving(cmd.parse()?)),
            TrustCntDev::CMD_ID => Ok(In::TrustCntDev(cmd.parse()?)),
            _ => Err(Error::unimplemented(&cmd)),
        }
    }
}
