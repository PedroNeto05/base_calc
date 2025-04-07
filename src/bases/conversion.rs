use super::operations;

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

fn to_base_10_float_part(float_part: String, base_to: u8) -> f64 {
    let mut sum: f64 = 0.0;
    let digits: Vec<char> = float_part.chars().collect();

    for (i, digit) in digits.iter().enumerate() {
        let digit_value = char_to_digit(digit).unwrap();
        let exp: i32 = -(i as i32 + 1);
        sum += digit_value as f64 * (base_to as f64).powi(exp);
    }

    // 0.123
    sum
}

pub fn to_base_10(num: &str, base_to: u8) -> f64 {
    let parts: Vec<&str> = num.split('.').collect();
    let is_negative = num.starts_with('-');
    let int_part = if is_negative {
        &parts[0][1..]
    } else {
        parts[0]
    };
    let float_part = if parts.len() > 1 { parts[1] } else { "" };

    let int_part = int_part.to_string();
    let float_part = float_part.to_string();

    let int_part = to_base_10_int_part(int_part, base_to);
    let float_part = to_base_10_float_part(float_part, base_to);

    let result = int_part + float_part;
    if is_negative { -result } else { result }
}

pub fn euclidean(mut numero: f64, base: u8) -> String {
    if base < 2 || base > 36 {
        panic!("Base precisa estar entre 2 e 36");
    }

    let caracteres = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect::<Vec<_>>();
    let mut resultado = String::new();

    if numero == 0.0 {
        return "0".to_string();
    }

    let negativo = numero < 0.0;
    numero = numero.abs();

    let parte_inteira = numero.floor() as u64;
    let mut parte_int = parte_inteira;
    let mut parte_inteira_str = String::new();

    if parte_int == 0 {
        parte_inteira_str.push('0');
    } else {
        while parte_int > 0 {
            let resto = (parte_int % base as u64) as usize;
            parte_inteira_str.insert(0, caracteres[resto]);
            parte_int /= base as u64;
        }
    }

    let mut parte_fracionaria = numero - (parte_inteira as f64);
    let mut parte_fracionaria_str = String::new();
    let mut contador = 0;
    let limite_maximo = 64;

    if parte_fracionaria > 0.0 {
        parte_fracionaria_str.push('.');
        while parte_fracionaria > 0.0 && contador < limite_maximo {
            parte_fracionaria *= base as f64;
            let digito = parte_fracionaria.floor() as usize;
            parte_fracionaria_str.push(caracteres[digito]);
            parte_fracionaria -= digito as f64;
            contador += 1;
        }
    }

    resultado = parte_inteira_str + &parte_fracionaria_str;

    if negativo {
        resultado.insert(0, '-');
    }

    resultado
}

pub fn any_to_any(num: &str, base_from: u8, base_dest: u8) -> String {
    let mut result = "0".to_string();
    
    let base_from_str = euclidean(base_from as f64, base_dest);
    
    for c in num.chars() {
        let digit_value = char_to_digit(&c).unwrap() as f64;
        let digit_str = euclidean(digit_value, base_dest);
        
        result = operations::prod(
            &result, 
            base_dest, 
            &base_from_str, 
            base_dest, 
            base_dest
        );
        
        result = operations::sum(
            &result, 
            base_dest, 
            &digit_str, 
            base_dest, 
            base_dest
        );
    }
    
    result
}
