use crate::frame::{
    frame_type::FrameType,
    header::{Flags, FrameHeader},
    setting::Setting,
    FrameErrors,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FramePayload {
    Data {
        padding: u8,
        content: Vec<u8>,
        last: bool,
    },
    Headers {
        padding: u8,
        priority: u8,
        exclusive: bool,
        stream_id: u32,
        end: bool,
        continuation: bool,
        fragments: Vec<u8>,
    },
    Settings(Vec<Setting>),
    Raw(Vec<u8>),
}

impl FramePayload {
    pub fn parse(header: &FrameHeader, payload: &[u8]) -> Result<FramePayload, FrameErrors> {
        match header.frame_type {
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

            FrameType::HEADERS => {
                let continuation = header.check_flag(&Flags::END_HEADERS);
                let end = header.check_flag(&Flags::END_STREAM);

                let mut start = 0;

                let padded_flag = header.check_flag(&Flags::PADDED);
                let priority_flag = header.check_flag(&Flags::PRIORITY);

                let padding = if padded_flag {
                    start += 1;
                    payload[0]
                } else {
                    0
                };

                let mut exclusive = false;
                let mut stream_id = 0_u32;

                let priority = if priority_flag {
                    let index = if padded_flag { 1 } else { 0 };
                    let raw_bytes = u32::from_be_bytes([
                        payload[index],
                        payload[index + 1],
                        payload[index + 2],
                        payload[index + 3],
                    ]);
                    exclusive = ((raw_bytes & 0x80000000) >> 31) != 0;
                    stream_id = raw_bytes & 0x7fffffff;

                    start += 5;
                    payload[index + 4]
                } else {
                    0
                };

                if start + (padding as usize) >= payload.len() {
                    return Err(FrameErrors::ProtocolError);
                }

                let fragments = payload[start..(payload.len() - padding as usize)].to_vec();
                Ok(FramePayload::Headers {
                    padding,
                    priority,
                    exclusive,
                    stream_id,
                    end,
                    continuation,
                    fragments,
                })
            }

            FrameType::SETTINGS => {
                let ack_flag = header.check_flag(&Flags::ACK);
                if ack_flag && payload.len() != 0 {
                    return Err(FrameErrors::FrameSizeError);
                }

                let settings = Setting::parse(payload)?;
                Ok(FramePayload::Settings(settings))
            }
            _ => Ok(FramePayload::Raw(payload.to_vec())),
        }
    }
}
