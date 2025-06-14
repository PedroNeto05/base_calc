mod bases;
mod utils;

fn main() {
    //conversão
    let number_in_b1 = utils::input("Digite um numero para ser transformado para a b10: ");
    let base_to = utils::input("Digite sua base: ").parse::<u8>().unwrap();
    let number_in_b10 = bases::conversion::to_base_10(&number_in_b1, base_to);
    println!("numero na b10: {number_in_b10}");


    println!();
    println!();
    println!();
    println!();
    println!();
    println!();

    // operações
    let base1 = utils::input("Digite a base do primeiro numero: ")
        .parse::<u8>()
        .unwrap();
    let num1 = utils::input("Digite o primeiro numero: ");
    let base2 = utils::input("Digite a base do segundo numero: ")
        .parse::<u8>()
        .unwrap();
    let num2 = utils::input("Digite o segundo numero: ");

    let base = 10;
    let sum_result = bases::operations::sum(&num1, base1, &num2, base2, base);
    println!("Soma dos numeros: {sum_result} na base 10");

    let prod_result = bases::operations::prod(&num1, base1, &num2, base2, base);
    println!("Produto dos numeros: {prod_result} na base 10");

    println!();
    println!();
    println!();
    println!();
    println!();
    println!();

    let num = utils::input("Digite o número a ser convertido: ");
    let base_from = utils::input("Digite a base de origem: ").parse::<u8>().unwrap();
    let base_dest = utils::input("Digite a base de destino: ").parse::<u8>().unwrap();

    let result = bases::conversion::any_to_any(&num, base_from, base_dest);
    println!("Resultado da conversão: {result}");
}
