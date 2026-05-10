use crate::frame::FrameErrors;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrameType {
    DATA = 0x00,
    HEADERS = 0x01,
    PRIORITY = 0x02,
    RST_STREAM = 0x03,
    SETTINGS = 0x04,
    PUSH_PROMISE = 0x05,
    PING = 0x06,
    GOAWAY = 0x07,
    WINDOW_UPDATE = 0x08,
    CONTINUATION = 0x09,
}

impl TryFrom<u8> for FrameType {
    type Error = FrameErrors;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(FrameType::DATA),
            0x01 => Ok(FrameType::HEADERS),
            0x02 => Ok(FrameType::PRIORITY),
            0x03 => Ok(FrameType::RST_STREAM),
            0x04 => Ok(FrameType::SETTINGS),
            0x05 => Ok(FrameType::PUSH_PROMISE),
            0x06 => Ok(FrameType::PING),
            0x07 => Ok(FrameType::GOAWAY),
            0x08 => Ok(FrameType::WINDOW_UPDATE),
            0x09 => Ok(FrameType::CONTINUATION),
            _ => return Err(FrameErrors::UnknownTypeCode(value)),
        }
    }
}
