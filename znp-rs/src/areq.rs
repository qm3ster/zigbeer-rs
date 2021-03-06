use crate::cmd::error::Result;
use crate::znp_codec::{Subsys, Type, ZnpCmd};
use bytes::{buf::BufMutExt, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
pub trait AreqIn: DeserializeOwned {
    const SUBSYS: Subsys;
    const CMD_ID: u8;
    fn parse(res: ZnpCmd) -> Result<Self> {
        res.parse().map_err(From::from)
    }
}
pub trait AreqOut: Serialize {
    const SUBSYS: Subsys;
    const CMD_ID: u8;
    /// Serialized params size; <= 250
    const MAX_SIZE: usize;
    fn frame(&self) -> ZnpCmd {
        let mut body = BytesMut::with_capacity(Self::MAX_SIZE);
        let writer = (&mut body).writer();
        crate::serde_znp::serialize(writer, self).unwrap();
        ZnpCmd::new(Type::AREQ, Self::SUBSYS, Self::CMD_ID, body)
    }
}
