pub fn bit_is_set(value: u8, bit_number: u8) -> bool {
    ((value >> bit_number) & 0x01) == 0x01
}