pub enum CPUStatusFlags {
    CarryFlag = 0,
    ZeroResult = 1,
    InterruptDisable = 2,
    DecimalMode = 3,
    BreakFlag = 4,
    Break2Flag = 5,
    OverflowFlag = 6,
    NegativeResult = 7,
}

#[derive(Default)]
pub(crate) struct Registers {
    pub a: u8, // Accumulator
    pub x: u8, // Index Register X
    pub y: u8, // Index Register X
    pub p: u8, // Processor Status
    pub sp: u8, // Stack Pointer
    pub pc: u16, // Program Counter
}