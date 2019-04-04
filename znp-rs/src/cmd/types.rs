use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortAddr(u16);

#[derive(Serialize, Deserialize, Debug)]
pub struct IEEEAddr(u64);
