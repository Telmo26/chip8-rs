pub fn reconstruct_byte(bytes: &[usize]) -> u16 {
    let length = bytes.len();
    let mut result: u16 = 0;
    for i in 0..length {
        result = result | (bytes[length - 1 - i] << (i * 4)) as u16
    }
    result
}

pub fn byte_to_bools(byte: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..8 {
        bits[7 - i] = ((byte >> i) & 0b1) != 0;
    };
    bits
}