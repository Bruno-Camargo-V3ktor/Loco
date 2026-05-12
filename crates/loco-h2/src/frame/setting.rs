use crate::frame::FrameErrors;

const SIZE_PAYLOAD: usize = 6;

#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettingsId {
    HEADER_TABLE_SIZE = 0x01,
    ENABLE_PUSH = 0x02,
    MAX_CONCURRENT_STREAMS = 0x03,
    INITIAL_WINDOW_SIZE = 0x04,
    MAX_FRAME_SIZE = 0x05,
    MAX_HEADER_LIST_SIZE = 0x06,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Setting {
    pub id: SettingsId,
    pub value: u32,
}

impl TryFrom<u16> for SettingsId {
    type Error = FrameErrors;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::HEADER_TABLE_SIZE),
            0x02 => Ok(Self::ENABLE_PUSH),
            0x03 => Ok(Self::MAX_CONCURRENT_STREAMS),
            0x04 => Ok(Self::INITIAL_WINDOW_SIZE),
            0x05 => Ok(Self::MAX_FRAME_SIZE),
            0x06 => Ok(Self::MAX_HEADER_LIST_SIZE),
            _ => Err(FrameErrors::UnknownIdSettings(value)),
        }
    }
}

impl Setting {
    pub fn parse(payload: &[u8]) -> Result<Vec<Self>, FrameErrors> {
        if payload.len() % SIZE_PAYLOAD != 0 {
            return Err(FrameErrors::InvalidSettingsPlayload(payload.to_vec()));
        }

        let mut settings = Vec::with_capacity((payload.len() / SIZE_PAYLOAD) + 1);
        for chunk in payload.chunks_exact(SIZE_PAYLOAD) {
            let id = SettingsId::try_from(u16::from_be_bytes([chunk[0], chunk[1]]))?;
            let value = u32::from_be_bytes([chunk[2], chunk[3], chunk[4], chunk[5]]);

            settings.push(Setting { id, value });
        }

        Ok(settings)
    }
}

