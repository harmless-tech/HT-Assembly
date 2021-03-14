pub trait EnumWithU8 {
    fn to_u8(&self) -> u8;
    fn from_u8(e: u8) -> Self;
}
