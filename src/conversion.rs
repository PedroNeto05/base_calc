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

fn to_base_10_int_part(int_part: String, base: u8) -> f64 {
    let mut sum = 0;
    let digits: Vec<char> = int_part.chars().rev().collect();
    let base = base as i64;

    for (i, digit) in digits.iter().enumerate() {
        let digit_value = char_to_digit(digit).unwrap();
        sum += digit_value as i64 * base.pow(i as u32);
    }

    sum as f64
}

fn to_base_10_float_part(float_part: String, f: FloatPoint) -> f64 {
    let mut sum: f64 = 0.0;
    let digits: Vec<char> = float_part.chars().collect();

    for (i, digit) in digits.iter().enumerate() {
        let digit_value = char_to_digit(digit).unwrap();
        let exp: i32 = -(i as i32 + 1);
        sum += digit_value as f64 * (f.base_to as f64).powi(exp);
    }

    // 0.123
    sum
}

pub fn to_base_10(num: &str, f: FloatPoint) -> String {
    let parts: Vec<&str> = num.split('.').collect();
    let int_part = parts[0];
    let float_part = if parts.len() > 1 { parts[1] } else { "" };

    let int_part = int_part.to_string();
    let float_part = float_part.to_string();

    let int_part = to_base_10_int_part(int_part, f.base_to);
    let float_part = to_base_10_float_part(float_part, f);

    format!("{}", int_part + float_part)
}
