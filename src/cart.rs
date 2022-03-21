use crate::lib::get_bit;
use std::{fs::File, io::Read};

const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Debug)]
pub enum Mirroring {
   Vertical,
   Horizontal,
   FourScreen,
}

pub struct RomHeader {
    prg_rom_start: usize,
    chr_rom_start: usize,
    prg_rom_size: usize,
    chr_rom_size: usize,
    mapper: u8,
    pub screen_mirroring: Mirroring,    
}

impl RomHeader {
    fn new(buffer: &Vec<u8>) -> Self {
        if &buffer[0..4] != NES_TAG {
            panic!("File is not in iNES file format");
        }   

        let mapper = (buffer[7] & 0b1111_0000) | (buffer[6] >> 4);

        let ines_ver = (buffer[7] >> 2) & 0b11;
        if ines_ver != 0 {
            panic!("NES2.0 format is not supported");
        }

        let four_screen = get_bit(buffer[6], 3);
        let mirroring = get_bit(buffer[6], 0);
        let screen_mirroring = match (four_screen, mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };
 
        let prg_rom_size = buffer[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = buffer[5] as usize * CHR_ROM_PAGE_SIZE;

        let has_trainer = get_bit(buffer[6], 2);

        let prg_rom_start = 16 + if has_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        // Added 1 to each end index
        Self {
            prg_rom_start: prg_rom_start,
            chr_rom_start: chr_rom_start,
            prg_rom_size,
            chr_rom_size,
            mapper,
            screen_mirroring,   
        }
        
    }

}

#[allow(dead_code)]
pub(crate) struct Cart {
    pub filename: String,
    pub rom_size: usize,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub rom_header: RomHeader
}

impl Cart {
    pub(crate) fn new(file_path: &str) -> Self {
        let mut rom_data = Vec::new();
        let rom_size: usize;
        // Open file inside its own scope so it is dropped when file is read into buffer
        {
            let mut file = File::open(file_path).expect(format!("Unable to open file {}", file_path).as_str());
            // read the whole file
            rom_size = file.read_to_end(&mut rom_data).expect(format!("Unable to read file {}", file_path).as_str());
        }
        
        let header = RomHeader::new(&rom_data);
        let prg_rom = rom_data[header.prg_rom_start..(header.prg_rom_start + header.prg_rom_size)].to_vec();
        let chr_rom = rom_data[header.chr_rom_start..(header.chr_rom_start + header.chr_rom_size)].to_vec();

        Self{
            filename: file_path.to_string(),
            rom_size,
            prg_rom,
            chr_rom,
            rom_header: header
        }
    }
}

