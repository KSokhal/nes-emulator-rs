use crate::lib::{set_bit, get_bit};

#[derive(Clone, Copy)]
pub enum Inputs {
    Right,
    Left,
    Down,
    Up,
    Start,
    Select,
    B,
    A,
}

const RIGHT_FLAG_BYTE_POSITION: u8 = 7;
const LEFT_FLAG_BYTE_POSITION: u8 = 6;
const DOWN_FLAG_BYTE_POSITION: u8 = 5;
const UP_FLAG_BYTE_POSITION: u8 = 4;
const START_FLAG_BYTE_POSITION: u8 = 3;
const SELECT_FLAG_BYTE_POSITION: u8 = 2;
const BUTTON_B_FLAG_BYTE_POSITION: u8 = 1;
const BUTTON_A_FLAG_BYTE_POSITION: u8 = 0;


pub struct Joypad {
    strobe: bool,
    button_index: u8,
    button_status: u8,
}
 
impl Joypad {
    pub fn new() -> Self {
        Joypad {
            strobe: false,
            button_index: 0,
            button_status: 0,
        }
    }

    pub fn read(&mut self) -> u8 {
        if self.button_index > 7 {
            return 1;
        }
        let response = (self.button_status & (1 << self.button_index)) >> self.button_index;
        if !self.strobe && self.button_index <= 7 {
            self.button_index += 1;
        }
        response
    }

    pub fn write(&mut self, data: u8) {
        // self.strobe = data & 1 == 1;
        self.strobe = get_bit(data, 0);
        if self.strobe {
            self.button_index = 0
        }
    }

    pub fn set_button_pressed_status(&mut self, button: Inputs, pressed: bool) {
        let index = match button {
            Inputs::Right => 7,
            Inputs::Left => 6,
            Inputs::Down => 5,
            Inputs::Up => 4,
            Inputs::Start => 3,
            Inputs::Select => 2,
            Inputs::B => 1,
            Inputs::A => 0,
        };
        set_bit(&mut self.button_status, index, pressed);
    }
}