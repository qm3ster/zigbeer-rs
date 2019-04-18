use packed_struct::prelude::*;
use packed_struct_codegen::*;

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum FrameType {
    General = 0b00,
    Cluster = 0b01,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    ServerToClient = 0b0,
    ClientToServer = 0b1,
}
#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "msb0")]
pub struct FrameControl {
    #[packed_field(bits = "0..=1", ty = "enum")]
    frame_type: FrameType,
    #[packed_field(bits = "2")]
    manufacturer_specific: bool,
    #[packed_field(bits = "3", ty = "enum")]
    direction: Direction,
    #[packed_field(bits = "4")]
    disable_default_rsp: bool,
}
#[derive(Debug)]
pub struct ZclFrame {
    pub frame_control: FrameControl,
    pub manufacturer_code: Option<u16>,
    pub trans_seq_num: u8,
    pub cmd_id: u8,
    pub payload: Vec<u8>,
}
use bytes::Buf;
impl ZclFrame {
    pub fn parse<B: Buf>(mut buf: B) -> Self {
        let frame_control = FrameControl::unpack(&[buf.get_u8()]).unwrap();
        let ms = frame_control.manufacturer_specific;
        ZclFrame {
            frame_control,
            manufacturer_code: if ms { Some(buf.get_u16_le()) } else { None },
            trans_seq_num: buf.get_u8(),
            cmd_id: buf.get_u8(),
            payload: buf.collect(),
        }
    }
}
