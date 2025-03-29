pub struct FloatPoint {
    pub base_from: u8,
    pub base_to: u8,
    pub mantissa: u64,
    pub min_exp: i32,
    pub max_exp: i32,
}
fn char_to_digit(digit: &char) -> Option<u8> {
    match digit {
        '0'..='9' => Some(*digit as u8 - b'0'),
        'A'..='F' => Some(*digit as u8 - b'A' + 10),
        _ => panic!("Invalid char"),
    }
}
