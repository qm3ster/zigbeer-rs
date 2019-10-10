use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ShortAddr(pub u16);

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct IEEEAddr(pub u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Endpoint(pub u8);
