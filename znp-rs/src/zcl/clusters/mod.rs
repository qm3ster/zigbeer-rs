pub mod error;
pub mod general;

pub mod on_off;
pub mod rh_measure;
pub mod temp_measure;

use super::frame::{FrameType, ZclFrame};
use error::{Error, Result};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[repr(u16)]
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq)]
pub enum ClusterId {
    GenBasic = 0x0000,
    GenOnOff = 0x0006,
    /// Temperature Measurement
    TempMeasure = 0x0402,
    /// Relative Humidity Measurement
    RHMeasurement = 0x0405,
}
impl ClusterId {
    pub fn from(cluster_id: u16) -> Result<Self> {
        ClusterId::from_u16(cluster_id).ok_or_else(|| Error::unknown_cluster(cluster_id))
    }
}
#[derive(Debug)]
pub enum In {
    General(general::In),
    GenOnOff(on_off::In),
    TempMeasure(temp_measure::In),
    RHMeasurement(rh_measure::In),
}
impl In {
    pub fn parse(cluster: ClusterId, frame: ZclFrame) -> Result<Self> {
        use ClusterId::*;
        match frame.frame_control.frame_type {
            FrameType::General => Ok(In::General(general::In::from_cmd(frame)?)),
            FrameType::Cluster => match cluster {
                GenOnOff => Ok(In::GenOnOff(on_off::In::from_cmd(frame)?)),
                TempMeasure => Ok(In::TempMeasure(temp_measure::In::from_cmd(frame)?)),
                RHMeasurement => Ok(In::RHMeasurement(rh_measure::In::from_cmd(frame)?)),
                cluster => Err(Error::unimplemented_cluster(cluster)),
            },
        }
    }
}
