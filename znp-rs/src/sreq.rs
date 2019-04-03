use crate::znp_codec::{Subsys, Type, ZpiCmd};
use bytes::{BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
pub trait Sreq: Serialize {
    type Srsp: DeserializeOwned;
    const SUBSYS: Subsys;
    const CMD_ID: u8;
    /// Serialized params size; <= 250
    const MAX_SIZE: usize;
    fn frame(&self) -> ZpiCmd {
        let mut body = BytesMut::with_capacity(Self::MAX_SIZE);
        let writer = (&mut body).writer();
        crate::serde_znp::serialize(writer, self).unwrap();
        ZpiCmd::new(Type::SREQ, Self::SUBSYS, Self::CMD_ID, body)
    }
    fn parse_res(res: ZpiCmd) -> Self::Srsp {
        res.parse().unwrap()
    }
}
