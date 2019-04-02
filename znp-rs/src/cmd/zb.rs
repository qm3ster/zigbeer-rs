use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, Type, ZpiCmd};
use bytes::{BufMut, BytesMut};
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
    fn frame(&self) -> ZpiCmd {
        let typ = Type::SREQ;
        let subsys = Subsys::SAPI;
        let cmd_id = 0x06;
        let mut body = BytesMut::with_capacity(250);
        let writer = (&mut body).writer();
        crate::serde_znp::serialize(writer, self).unwrap();
        ZpiCmd::new(typ, subsys, cmd_id, body)
    }
}
