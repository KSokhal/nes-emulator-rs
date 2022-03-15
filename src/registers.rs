pub(crate) const CARRY_FLAG_BYTE_POSITION: u8 = 0;
pub(crate) const ZERO_RESULT_FLAG_BYTE_POSITION: u8 = 1;
pub(crate) const INTERRUPT_DISABLE_FLAG_BYTE_POSITION: u8 = 2;
pub(crate) const DECIMAL_MODE_FLAG_BYTE_POSITION: u8 = 3;
pub(crate) const BREAK_FLAG_BYTE_POSITION: u8 = 4;
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

impl Registers {
    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.p = 0;
    }
    // pub(crate) fn zero_flag(&self) -> bool {
    //     // Z Flag
    //     get_bit(self.f, 7)
    // }

    // pub(crate) fn subtract_flag(&self) -> bool {
    //     // N Flag
    //     get_bit(self.f, 6)
    // }

    // pub(crate) fn half_carry_flag(&self) -> bool {
    //     // H Flag
    //     get_bit(self.f, 5)
    // }

    // pub(crate) fn carry_flag(&self) -> bool {
    //     // C Flag
    //     get_bit(self.f, 4)
    // }
    // pub(crate) fn set_zero_flag(&mut self, value: bool) {
    //     // Z Flag
    //     set_bit(&mut self.f, 7, value);
    // }

    // pub(crate) fn set_subtract_flag(&mut self, value: bool) {
    //     // N Flag
    //     set_bit(&mut self.f, 6, value);
    // }

    // pub(crate) fn set_half_carry_flag(&mut self, value: bool) {
    //     // H Flag
    //     set_bit(&mut self.f, 5, value);
    // }

    // pub(crate) fn set_carry_flag(&mut self, value: bool) {
    //     // C Flag
    //     set_bit(&mut self.f, 4, value);
    // }

}


// struct FlagsRegister {
//     carry: bool,
//     zero: bool,
//     interrupt_disable: bool,
//     decimal_mode: bool,
//     break_flag: bool,
//     overflow: bool,
//     negative: bool,
// }


// impl std::convert::From<FlagsRegister> for u8  {
//     fn from(flag: FlagsRegister) -> u8 {
//         (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
//         (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
//         (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
//         (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
//     }
// }

// impl std::convert::From<u8> for FlagsRegister {
//     fn from(byte: u8) -> Self {
//         let carry = ((byte >> 0) & 0b1) != 0;
//         let zero = ((byte >> 1) & 0b1) != 0;
//         let interrupt_disable = ((byte >> 2) & 0b1) != 0;
//         let decimal_mode = ((byte >> 3) & 0b1) != 0;
//         let break_flag = ((byte >> 4) & 0b1) != 0;
//         let overflow = ((byte >> 6) & 0b1) != 0;
//         let negative = ((byte >> 7) & 0b1) != 0;

//         FlagsRegister {
//             carry,
//             zero,
//             interrupt_disable,
//             decimal_mode,
//             break_flag,
//             overflow,
//             negative,
//         }
//     }
// }