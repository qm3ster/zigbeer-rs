use crate::sreq::Sreq;
use crate::znp_codec::{Subsys, Type, ZpiCmd};
use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};

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
    fn frame(&self) -> ZpiCmd {
        let typ = Type::SREQ;
        let subsys = Subsys::SYS;
        let cmd_id = 0x0A;
        let mut body = BytesMut::with_capacity(250);
        let writer = (&mut body).writer();
        crate::serde_znp::serialize(writer, self).unwrap();
        ZpiCmd::new(typ, subsys, cmd_id, body)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerExpired {
    pub timer_id: u8,
}
