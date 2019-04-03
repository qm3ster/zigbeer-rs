pub mod error;
use error::{Error, Result};
pub mod sys;
pub mod util;
pub mod zb;
use crate::znp_codec::{Subsys, ZpiCmd};
#[derive(Debug)]
pub enum Areq {
    Sys(sys::In),
}
impl Areq {
    pub fn from_subsys(cmd: ZpiCmd) -> Result<Self> {
        use Areq::*;
        use Subsys::*;
        match cmd.subsys() {
            SYS => Ok(Sys(sys::In::from_cmd(cmd)?)),
            _ => Err(Error::Subsys(cmd.subsys())),
        }
    }
}
