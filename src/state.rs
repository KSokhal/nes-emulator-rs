use serde_big_array::BigArray;

use crate::ppu::PPU;

#[derive(Deserialize, Serialize)]
struct State {
    #[serde(with = "BigArray")]
    ram: [u8; 2048],
    ppu: PPU,
}

impl State {
    fn new() {

    }

    fn save_state() {

    }
    
    fn load_state() {

    }
}