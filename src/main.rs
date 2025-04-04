mod utils;
mod bases;

fn main() {
    let base_to = utils::input("Digite uma base para transformar o numero: ").parse::<u8>().unwrap();
    let number_in_b1 = utils::input("Digite um numero em uma base b1: ");
    let number_in_b10 = bases::conversion::to_base_10(&number_in_b1, base_to);
    println!("numero na b10: {number_in_b10}");

    let base1 = utils::input("Digite a base do primeiro numero: ").parse::<u8>().unwrap();
    let num1 = utils::input("Digite o primeiro numero: ");
    let base2 = utils::input("Digite a base do segundo numero: ").parse::<u8>().unwrap();
    let num2 = utils::input("Digite o segundo numero: ");

    let sum_result = bases::operations::sum(&num1, base1, &num2, base2);
    println!("Soma dos numeros: {sum_result} na base 10");

    let prod_result = bases::operations::prod(&num1, base1, &num2, base2);
    println!("Produto dos numeros: {prod_result} na base 10");
}


