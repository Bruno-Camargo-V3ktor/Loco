use crate::frame::{
    frame_type::FrameType,
    header::{Flags, FrameHeader},
    settings::Setting,
    FrameErrors,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FramePayload {
    Settings(Vec<Setting>),
    Data {
        padding: u8,
        content: Vec<u8>,
        last: bool,
    },
    Raw(Vec<u8>),
}

impl FramePayload {
    pub fn parse(header: &FrameHeader, payload: &[u8]) -> Result<FramePayload, FrameErrors> {
        match header.frame_type {
            FrameType::SETTINGS => {
                let ack_flag = header.check_flag(&Flags::ACK);
                if ack_flag && payload.len() != 0 {
                    return Err(FrameErrors::FrameSizeError);
                }

                let settings = Setting::parse(payload)?;
                Ok(FramePayload::Settings(settings))
            }

            FrameType::DATA => {
                let padding = if header.check_flag(&Flags::PADDED) {
                    payload[0]
                } else {
                    0
                };

                if padding as usize >= payload.len() {
                    return Err(FrameErrors::ProtocolError);
                }

                let last = header.check_flag(&Flags::END_STREAM);

                let start = 0 + if padding != 0 { 1 } else { 0 };
                let end = payload.len() - padding as usize;
                let content = payload[start..end].to_vec();

                Ok(FramePayload::Data {
                    padding,
                    content,
                    last,
                })
            }

            _ => Ok(FramePayload::Raw(payload.to_vec())),
        }
    }
}
