use crate::znp_codec::ZpiCmd;
use serde::de::DeserializeOwned;
pub trait Sreq {
    type Srsp: DeserializeOwned;
    fn frame(&self) -> ZpiCmd;
    fn parse_res(res: ZpiCmd) -> Self::Srsp {
        res.parse().unwrap()
    }
}
