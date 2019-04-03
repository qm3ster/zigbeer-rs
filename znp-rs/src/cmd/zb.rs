use crate::sreq::Sreq;
use crate::znp_codec::Subsys;

use serde::{Deserialize, Serialize};

use serde_repr::{Deserialize_repr, Serialize_repr};
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ZbDeviceInfoProp {
    DevState = 0,
    IeeeAddr = 1,
    ShortAddr = 2,
    ParentShortAddr = 3,
    ParentIeeeAddr = 4,
    Channel = 5,
    PanId = 6,
    ExtPanId = 7,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ZbGetDeviceInfoReq {
    pub param: ZbDeviceInfoProp,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ZbGetDeviceInfoRsp {
    pub param: ZbDeviceInfoProp,
    pub value: [u8; 8],
}
impl Sreq for ZbGetDeviceInfoReq {
    type Srsp = ZbGetDeviceInfoRsp;
    const SUBSYS: Subsys = Subsys::SAPI;
    const CMD_ID: u8 = 0x06;
    const MAX_SIZE: usize = 9;
}
