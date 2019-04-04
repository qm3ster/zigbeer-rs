pub mod error;
use error::{Error, Result};
pub mod types;

pub mod sys;
pub mod util;
pub mod zb;
pub mod zdo;
use crate::znp_codec::{Subsys, ZpiCmd};
#[derive(Debug)]
pub enum Areq {
    Sys(sys::In),
    Zdo(zdo::In),
}
impl Areq {
    pub fn from_subsys(cmd: ZpiCmd) -> Result<Self> {
        use Areq::*;
        use Subsys::*;
        match cmd.subsys() {
            SYS => Ok(Sys(sys::In::from_cmd(cmd)?)),
            ZDO => Ok(Zdo(zdo::In::from_cmd(cmd)?)),
            _ => Err(Error::mismatched(&cmd)),
        }
    }
}
