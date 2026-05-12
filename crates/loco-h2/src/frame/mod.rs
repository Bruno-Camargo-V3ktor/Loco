pub mod frame_type;
pub mod header;
pub mod payload;
pub mod settings;

use header::FrameHeader;
use payload::FramePayload;

#[derive(Clone, Debug)]
pub enum FrameErrors {
    UnknownTypeCode(u8),
    UnknownIdSettings(u16),
    InvalidSettingsPlayload(Vec<u8>),
    FrameSizeError,
    ProtocolError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub header: header::FrameHeader,
    pub payload: payload::FramePayload,
}

impl Frame {
    pub fn new(payload: &[u8]) -> Result<Self, FrameErrors> {
        let header = FrameHeader::parse(&[
            payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
            payload[7], payload[8],
        ])?;
        let payload = FramePayload::parse(&header, &payload[9..])?;

        Ok(Self { header, payload })
    }
}
