use crate::sreq::Sreq;
use crate::znp_codec::Subsys;

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
    const SUBSYS: Subsys = Subsys::UTIL;
    const CMD_ID: u8 = 0x0A;
    const MAX_SIZE: usize = 2;
}
