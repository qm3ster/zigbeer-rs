use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortAddr(pub u16);

#[derive(Serialize, Deserialize, Debug)]
pub struct IEEEAddr(pub u64);
