use crate::frame::{
    FrameErrors,
    header::{Flags, FrameHeader},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub padding: u8,
    pub content: Vec<u8>,
    pub last: bool,
}

impl Data {
    pub fn parse(frame_header: &FrameHeader, payload: &[u8]) -> Result<Self, FrameErrors> {
        let padding = if frame_header.check_flag(&Flags::PADDED) {
            payload[0]
        } else {
            0
        };

        if padding as usize >= payload.len() {
            return Err(FrameErrors::ProtocolError);
        }

        let last = frame_header.check_flag(&Flags::END_STREAM);

        let start = 0 + if padding != 0 { 1 } else { 0 };
        let end = payload.len() - padding as usize;
        let content = payload[start..end].to_vec();

        Ok(Self {
            padding,
            content,
            last,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.content.len() + self.padding as usize);

        if self.padding != 0 {
            bytes[0] = self.padding;
        }

        bytes.append(&mut self.content.clone());
        bytes.append(&mut vec![0; self.padding as usize]);

        bytes
    }
}
