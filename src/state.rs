use std::{fs::File, io::{Write, BufReader}};

use serde_big_array::BigArray;

use crate::ppu::PPU;

#[derive(Deserialize, Serialize)]
pub struct State {
    #[serde(with = "BigArray")]
    pub ram: [u8; 2048],
    pub ppu: PPU,
}

impl State {
    pub fn new(ram: [u8; 2048], ppu: PPU) -> Self {
        Self {
            ram,
            ppu
        }
    }

    pub fn save_state(self, state_number: u8) {
        let mut file = File::create(format!("save-state-{}", state_number)).expect("Failed to save state to file");
        let _ = file.write_all(&rmp_serde::to_vec(&self).unwrap());
    }
    
    pub fn load_state(state_number: u8) -> Self {
        let mut file = File::open("save_state_1").unwrap();
        let reader = BufReader::new(file);
        let s: State = rmp_serde::from_read(reader).unwrap();
        s
    }
}