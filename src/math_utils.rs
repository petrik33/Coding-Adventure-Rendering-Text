pub fn flag_is_on(bit_mask: u8, flag: u8) -> bool {
    (bit_mask & flag) == flag
}
