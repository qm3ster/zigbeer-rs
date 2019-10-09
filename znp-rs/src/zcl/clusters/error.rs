use super::ClusterId;
#[derive(Debug)]
pub enum Error {
    UnknownCmd { cmd_id: u8 },
    UnknownCluser { cluster: u16 },
    UnimplementedCluster { cluster: ClusterId },
    Payload(String),
}
impl Error {
    pub fn unknown_cmd(cmd_id: u8) -> Self {
        Error::UnknownCmd { cmd_id }
    }
    pub fn unknown_cluster(cluster: u16) -> Self {
        Error::UnknownCluser { cluster }
    }
    pub fn unimplemented_cluster(cluster: ClusterId) -> Self {
        Error::UnimplementedCluster { cluster }
    }
}

impl From<crate::serde_znp::Error> for Error {
    fn from(err: crate::serde_znp::Error) -> Self {
        Error::Payload(err.to_string())
    }
}
pub type Result<T> = std::result::Result<T, Error>;
