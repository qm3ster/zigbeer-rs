use crate::cmd::error::Result;
use crate::znp_codec::{Subsys, Type, ZnpCmd};
use bytes::{BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
pub trait Sreq: Serialize {
    type Srsp: DeserializeOwned;
    const SUBSYS: Subsys;
    const CMD_ID: u8;
    /// Serialized params size; <= 250
    const MAX_SIZE: usize;
    fn frame(&self) -> ZnpCmd {
        let mut body = BytesMut::with_capacity(Self::MAX_SIZE);
        let writer = (&mut body).writer();
        crate::serde_znp::serialize(writer, self).unwrap();
        ZnpCmd::new(Type::SREQ, Self::SUBSYS, Self::CMD_ID, body)
    }
    fn parse_res(res: ZnpCmd) -> Result<Self::Srsp> {
        res.parse().map_err(From::from)
    }
}
