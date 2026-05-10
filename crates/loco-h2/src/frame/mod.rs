pub mod frame_type;
pub mod header;

pub enum FrameErrors {
    UnknownTypeCode(u8),
}
