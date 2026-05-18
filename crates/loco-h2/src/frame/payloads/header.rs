use crate::frame::{
    FrameErrors,
    header::{Flags, FrameHeader},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub padding: u8,
    pub priority: u8,
    pub exclusive: bool,
    pub stream_id: u32,
    pub end: bool,
    pub continuation: bool,
    pub fragments: Vec<u8>,
}

impl Header {
    pub fn parse(frame_header: &FrameHeader, payload: &[u8]) -> Result<Self, FrameErrors> {
        let continuation = frame_header.check_flag(&Flags::END_HEADERS);
        let end = frame_header.check_flag(&Flags::END_STREAM);

        let mut start = 0;

        let padded_flag = frame_header.check_flag(&Flags::PADDED);
        let priority_flag = frame_header.check_flag(&Flags::PRIORITY);

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
        Ok(Self {
            padding,
            priority,
            exclusive,
            stream_id,
            end,
            continuation,
            fragments,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.fragments.len() + self.padding as usize + 5);

        if self.padding != 0 {
            bytes.push(self.padding);
        }

        if self.exclusive {
            let value = self.stream_id | ((self.exclusive as u32) << 31);
            let mut exclusive = value.to_be_bytes().to_vec();
            bytes.append(&mut exclusive);
        }

        bytes.append(&mut self.fragments.clone());
        bytes.append(&mut vec![0; self.padding as usize]);
        bytes
    }
}
