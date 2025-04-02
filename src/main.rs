mod conversion;
mod utils;

fn main() {
    let base_to = utils::input("Digite uma base para transformar o numero: ").parse::<u8>().unwrap();
    let number_in_b1 = utils::input("Digite um numero em uma base b1: ");
    let number_in_b10 = conversion::to_base_10(&number_in_b1, base_to);
    println!("numero na b10: {number_in_b10}")
}
