use std::io::{self, Write};
pub fn input(input_msg: &str) -> String {
    print!("{input_msg}");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Something went wrong");

    user_input.trim().to_string()
}
