/// # Little Endian u32 to u8
///
/// Converts a number in u32 format into its little endian u8 array representation.
///
pub fn le_u32_to_u8(i: u32) -> [u8;4] {
    return [((i << 24) & 0xFF) as u8,
        ((i << 16) & 0xFF) as u8,
        ((i << 8) & 0xFF) as u8,
        (i & 0xFF) as u8];
}
