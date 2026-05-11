/// Representa o cabeçalho fixo de 9 bytes que inicia todo frame HTTP/2.
///
/// Conforme RFC 9113, Seção 4.1 (Frame Format):
///
/// +-----------------------------------------------+
/// |                 Length (24)                   |
/// +---------------+---------------+---------------+
/// |   Type (8)    |   Flags (8)   |
/// +-+-------------+---------------+-------------------------------+
/// |R|                 Stream Identifier (31)                      |
/// +=+=============================================================+
/// |                   Frame Payload (0...)                      ...
/// +---------------------------------------------------------------+
///
/// R: Um único bit reservado. A semântica deste bit é indefinida, e a
///    RFC exige que ele DEVE permanecer zerado (0x0) ao enviar e
///    DEVE ser ignorado ao receber.
use crate::frame::{frame_type::FrameType, FrameErrors};

#[allow(non_camel_case_types)]
pub const ACK_FLAG: u8 = 1;
pub const END_STREAM_FLAG: u8 = 1;
pub const END_HEADERS_FLAG: u8 = 4;
pub const PADDED_FLAG: u8 = 8;
pub const PRIORITY_FLAG: u8 = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameHeader {
    pub length: u32,           // 24 Bits
    pub frame_type: FrameType, // 8 Bits
    pub flags: u8,             // 8 Bits
    pub stream_id: u32,        // 31 Bits
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Flags {
    ACK,
    END_STREAM,
    END_HEADERS,
    PADDED,
    PRIORITY,
}

impl FrameHeader {
    pub const SIZE: usize = 9;

    pub fn parse(buf: &[u8; Self::SIZE]) -> Result<Self, FrameErrors> {
        let length = u32::from_be_bytes([0, buf[0], buf[1], buf[2]]);
        let frame_type = FrameType::try_from(buf[3])?;
        let flags = buf[4];
        let stream_id = u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]]) & 0x7FFFFFFF;

        Ok(Self {
            length,
            frame_type,
            flags,
            stream_id,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        let len_bytes = self.length.to_be_bytes();

        buf[0..3].copy_from_slice(&len_bytes[1..4]);
        buf[3] = self.frame_type.clone() as u8;
        buf[4] = self.flags;

        let stream_bytes = self.stream_id.to_be_bytes();
        buf[5..9].copy_from_slice(&stream_bytes);

        buf
    }

    pub fn check_flag(&self, flag: &Flags) -> bool {
        match flag {
            Flags::ACK => self.flags & ACK_FLAG != 0,

            Flags::END_STREAM => self.flags & END_STREAM_FLAG != 0,

            Flags::END_HEADERS => self.flags & END_HEADERS_FLAG != 0,

            Flags::PADDED => self.flags & PADDED_FLAG != 0,

            Flags::PRIORITY => self.flags & PRIORITY_FLAG != 0,
        }
    }

    pub fn get_flags(&self) -> Vec<Flags> {
        let mut flags = Vec::with_capacity(8);

        for flag in [
            Flags::ACK,
            Flags::END_STREAM,
            Flags::END_HEADERS,
            Flags::PADDED,
            Flags::PRIORITY,
        ] {
            if self.check_flag(&flag) {
                flags.push(flag);
            }
        }

        flags
    }
}
