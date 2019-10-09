use super::super::frame::ZclFrame;
use super::error::{Error, Result};

#[derive(Debug)]
pub enum In {}
impl In {
    pub fn from_cmd(cmd: ZclFrame) -> Result<Self> {
        match cmd.cmd_id {
            _ => {
                println!("{:x?}", cmd);
                Err(Error::unknown_cmd(cmd.cmd_id))
            }
        }
    }
}
