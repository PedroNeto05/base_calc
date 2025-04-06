use super::conversion;

pub fn sum(num1: &str, base1: u8, num2: &str, base2: u8, base_dest: u8) -> String {
    let result = conversion::to_base_10(num1, base1) + conversion::to_base_10(num2, base2);
    conversion::euclidean(result, base_dest)
}

pub fn prod(num1: &str, base1: u8, num2: &str, base2: u8, base_dest: u8) -> String {
    let result = conversion::to_base_10(num1, base1) * conversion::to_base_10(num2, base2);
    conversion::euclidean(result, base_dest)
}
