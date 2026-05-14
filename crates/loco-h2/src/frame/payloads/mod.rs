pub mod data;
pub mod header;
pub mod setting;

use crate::frame::{
    FrameErrors,
    frame_type::FrameType,
    header::{Flags, FrameHeader},
    payloads::{data::Data, header::Header, setting::Setting},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FramePayload {
    Data(Data),
    Headers(Header),
    Settings(Vec<Setting>),
    Raw(Vec<u8>),
}

impl FramePayload {
    pub fn parse(header: &FrameHeader, payload: &[u8]) -> Result<FramePayload, FrameErrors> {
        match header.frame_type {
            FrameType::DATA => Ok(FramePayload::Data(Data::parse(&header, payload)?)),

            FrameType::HEADERS => Ok(FramePayload::Headers(Header::parse(&header, payload)?)),

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

impl FramePayload {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Data(data) => data.to_bytes(),

            Self::Headers(_) => todo!(),

            Self::Settings(settings) => Setting::bytes(settings),

            Self::Raw(bytes) => bytes.clone(),
        }
    }
}
