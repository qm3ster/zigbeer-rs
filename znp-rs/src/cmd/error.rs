use crate::znp_codec::{Subsys, ZnpCmd};
#[derive(Debug)]
pub enum Error {
    Unimplemented { subsys: Subsys, cmd_id: u8 },
    Mismatched { subsys: Subsys, cmd_id: u8 },
    Payload(String),
}
impl Error {
    pub fn unimplemented(cmd: &ZnpCmd) -> Self {
        Error::Unimplemented {
            subsys: cmd.subsys(),
            cmd_id: cmd.cmd_id(),
        }
    }
    pub fn mismatched(cmd: &ZnpCmd) -> Self {
        Error::Mismatched {
            subsys: cmd.subsys(),
            cmd_id: cmd.cmd_id(),
        }
    }
}

impl From<crate::serde_znp::Error> for Error {
    fn from(err: crate::serde_znp::Error) -> Self {
        Error::Payload(err.to_string())
    }
}
pub type Result<T> = std::result::Result<T, Error>;
