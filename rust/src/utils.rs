pub fn sign_extend(x: u16, bit_count: u8) -> u16 {
    let mut result = x;

    if ((x >> (bit_count - 1)) & 1) == 1 {
        result |= 0xFFF << bit_count;
    }

    result
}
