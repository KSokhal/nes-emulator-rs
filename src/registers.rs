pub(crate) const CARRY_FLAG_BYTE_POSITION: u8 = 0;
pub(crate) const ZERO_RESULT_FLAG_BYTE_POSITION: u8 = 1;
pub(crate) const INTERRUPT_DISABLE_FLAG_BYTE_POSITION: u8 = 2;
pub(crate) const DECIMAL_MODE_FLAG_BYTE_POSITION: u8 = 3;
pub(crate) const BREAK_FLAG_BYTE_POSITION: u8 = 4;
pub(crate) const BREAK2_FLAG_BYTE_POSITION: u8 = 5;
pub(crate) const OVERFLOW_FLAG_BYTE_POSITION: u8 = 6;
pub(crate) const NEGATIVE_RESULT_FLAG_BYTE_POSITION: u8 = 7;

#[derive(Default)]
pub(crate) struct Registers {
    pub a: u8, // Accumulator
    pub x: u8, // Index Register X
    pub y: u8, // Index Register X
    pub p: u8, // Processor Status
    pub sp: u8, // Stack Pointer
    pub pc: u16, // Program Counter
}