use std::io;
use std::io::stdin;
use std::process::exit;

const VERSION: &str = "0.1.1 2023-01-26";

fn main() {
    println!("v{VERSION}\n\n< Pretzel >\n");
    println!("[ C ] Create new project");
    println!("[ X ] Exit");
    match read_selection("cCxX") {
        'c' => println!("Creating project"),
        'x' => exit(0),
        _ => println!("Error!"),
    };
}

fn read_selection(accepted_inputs:&str) -> char {
    let input_chars: Vec<char> = accepted_inputs.chars().collect();
    loop {
        let buffer_chars: Vec<char> = read().chars().collect();
        if input_chars.contains(&buffer_chars[0]) {
            return buffer_chars[0];
        }
        println!("Wrong input! Please input one of the displayed characters.");
    }
}

fn read() -> String {
    let mut buffer: String = "".to_string();
    stdin().read_line(&mut buffer)
        .expect("Error reading input");
    buffer
}

