use crate::sreq::Sreq;
use crate::znp_codec::Subsys;
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
    const SUBSYS: Subsys = Subsys::SYS;
    const CMD_ID: u8 = 0x0A;
    const MAX_SIZE: usize = 3;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerExpired {
    pub timer_id: u8,
}
