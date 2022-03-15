#[allow(dead_code)]
pub(crate) fn get_bit(input: u8, index: u8) -> bool {
    input & (1 << index) != 0
}

#[allow(dead_code)]
pub(crate) fn set_bit(input: &mut u8, index: u8, value: bool) {
    let mut x = 1 << index;
    if value {
        *input |= x; // Set the bit
    } else {
        x = !x; // Flip all bits
        *input &= x; // Unset the bit
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_get_bit() {
        let bytes: u8 = 0b1001_1010;
        assert_eq!(get_bit(bytes, 0), false);
        assert_eq!(get_bit(bytes, 1), true);
    }

    #[test]
    #[should_panic]
    fn test_invalid_get_bit() {
        let bytes: u8 = 0b1001_1010;
        get_bit(bytes, 16);
    }

    #[test]
    fn test_valid_set_bit() {
        let mut bytes: u8 = 0b1001_1010;
        set_bit(&mut bytes, 2, true);
        assert_eq!(bytes, 0b1001_1110);

        let mut bytes: u8 = 0b1001_1010;
        set_bit(&mut bytes, 7, false);
        assert_eq!(bytes, 0b0001_1010);

        let mut bytes: u8 = 0b1001_1010;
        set_bit(&mut bytes, 0, false);
        assert_eq!(bytes, 0b1001_1010);

        let mut bytes: u8 = 0b1001_1010;
        set_bit(&mut bytes, 1, true);
        assert_eq!(bytes, 0b1001_1010);
    }

    #[test]
    #[should_panic]
    fn test_invalid_set_bit() {
        let mut bytes: u8 = 0b1001_1010;
        set_bit(&mut bytes, 16, true);
    }
}