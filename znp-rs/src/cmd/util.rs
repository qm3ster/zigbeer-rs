use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, Type, ZpiCmd};
use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UtilLedControl {
    pub led_id: u8,
    pub mode: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UtilLedControlRsp {
    pub status: u8,
}
impl Sreq for UtilLedControl {
    type Srsp = UtilLedControlRsp;
    fn frame(&self) -> ZpiCmd {
        let typ = Type::SREQ;
        let subsys = Subsys::UTIL;
        let cmd_id = 0x0A;
        let mut body = BytesMut::with_capacity(250);
        let writer = (&mut body).writer();
        crate::serde_znp::serialize(writer, self).unwrap();
        ZpiCmd::new(typ, subsys, cmd_id, body)
    }
}
