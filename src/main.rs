mod conversion;
mod utils;

fn main() {
    let number_in_b1 = utils::input("Digite um numero em uma base b1: ");
    let base_to = utils::input("Digite uma base para transformar o numero: ").parse::<u8>().unwrap();
    let f = conversion::FloatPoint {
        base_to,
        mantissa: 3,
        min_exp: 1,
        max_exp: 1,
        base_from: 10,
    };
    let number_in_b10 = conversion::to_base_10(&number_in_b1, f);
    println!("numero na b10: {number_in_b10}")
}
