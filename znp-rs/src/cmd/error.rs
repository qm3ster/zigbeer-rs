use crate::znp_codec::Subsys;
#[derive(Debug)]
pub enum Error {
    Subsys(Subsys),
    CmdId(u8),
    Payload(String),
}
impl From<crate::serde_znp::Error> for Error {
    fn from(err: crate::serde_znp::Error) -> Self {
        Error::Payload(err.to_string())
    }
}
pub type Result<T> = std::result::Result<T, Error>;
