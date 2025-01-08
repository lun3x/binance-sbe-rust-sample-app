#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExecutionType {
    New = 0x0_u8,
    Canceled = 0x1_u8,
    Replaced = 0x2_u8,
    Rejected = 0x3_u8,
    Trade = 0x4_u8,
    Expired = 0x5_u8,
    TradePrevention = 0x8_u8,
    Unknown = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ExecutionType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::New,
            0x1_u8 => Self::Canceled,
            0x2_u8 => Self::Replaced,
            0x3_u8 => Self::Rejected,
            0x4_u8 => Self::Trade,
            0x5_u8 => Self::Expired,
            0x8_u8 => Self::TradePrevention,
            0xfe_u8 => Self::Unknown,
            _ => Self::NullVal,
        }
    }
}
