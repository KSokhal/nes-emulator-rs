use std::fmt;

pub(crate) struct GameState {
    pub direction: u8,
    player_state: u8,
    float_state: u8,
    horizontal_speed: u8,
    horizontal_level_pos: u8,
    screen_x_pos: u8,
}

impl GameState {
    pub(crate) fn new(ram: &[u8; 2048]) -> Self {
        GameState {
            direction: ram[0x0003],
            player_state: ram[0x000E],
            float_state: ram[0x001D],
            horizontal_speed: ram[0x0057],
            horizontal_level_pos: ram[0x006D],
            screen_x_pos: ram[0x0086],
        }
    }

    pub fn read_direction(&self) -> &str {
        match self.direction {
            1 => "Right",
            2 => "Left",
            _ => "INVALID DIRECTION",
        }
    }

    fn read_player_state(&self) -> &str {
        match self.player_state {
            0x00 => "Leftmost of screen",
            0x01 => "Climbing vine",
            0x02 => "Entering reversed-L pipe",
            0x03 => "Going down a pipe",
            0x04 => "Autowalk",
            0x05 => "Autowalk",
            0x06 => "Player dies",
            0x07 => "Entering area",
            0x08 => "Normal",
            0x09 => "Transforming from Small to Large (cannot move)",
            0x0A => "Transforming from Large to Small (cannot move)",
            0x0B => "Dying",
            0x0C => "Transforming to Fire Mario (cannot move)",
            _ => "INVALID PLAYER STATE"
        }
    }

    fn read_float_state(&self) -> &str {
        match self.float_state {
            0x00 => "Standing on solid/else",
            0x01 => "Airborn by jumping",
            0x02 => "Airborn by walking of a ledge",
            0x03 => "Sliding down flagpole",
            _ => "INVALID FLOAT STATE",
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
        "{{
        \n\t Direction: {}
        \n\t Player State: {}
        \n\t Float State: {}
        \n\t Speed: {}
        \n\t Level Pos: {}
        \n\t X Pos: {}
        \n}}", 
        self.read_direction(),
        self.read_player_state(),
        self.read_float_state(),
        self.horizontal_speed,
        self.horizontal_level_pos,
        self.screen_x_pos,
    )
    }
}
