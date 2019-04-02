use serde::{Deserialize, Serialize};

mod ser;
pub use ser::Serializer;
mod de;
pub use de::Deserializer;
mod error;
pub use error::{Error, Result};

use std::io::Write;

pub fn serialize<W, T>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    let mut ser = Serializer::new(writer);
    Serialize::serialize(value, &mut ser)
}

pub fn deserialize<'de, T>(bytes: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut de = Deserializer::new(bytes);
    Deserialize::deserialize(&mut de)
}
