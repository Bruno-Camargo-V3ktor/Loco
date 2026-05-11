pub mod frame_type;
pub mod header;
pub mod settings;

#[derive(Clone, Debug)]
pub enum FrameErrors {
    UnknownTypeCode(u8),
    UnknownIdSettings(u16),
    InvalidSettingsPlayload(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {}
