pub fn reconstruct_byte(high: u16, mid: u16, low: u16) -> u16 {
    high << 8 | mid << 4 | low
}

pub fn byte_to_bools(byte: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..8 {
        bits[7 - i] = ((byte >> i) & 0b1) != 0;
    };
    bits
}