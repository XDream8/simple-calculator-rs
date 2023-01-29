// for getting user input
use std::io;
use std::process::exit;

fn main() {
    loop {
        let stdin: io::Stdin = io::stdin();

        // get selected operation
        let operation: u8 = match select_operation(&stdin).trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a supported operation");
                continue;
            }
        };

        if operation > 5 {
            println!("That is not a supported operation");
            continue;
        } else if operation == 5 {
            exit(0);
        }

        // get first number
        let first_number: f64 = match select_number(&stdin, "First number").trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a supported operation");
                continue;
            }
        };

        // get second number
        let second_number: f64 = match select_number(&stdin, "Second number").trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a supported operation");
                continue;
            }
        };

        let result: f64 = result(first_number, second_number, operation);
        let operation_symbol: char = get_operation_symbol(operation);
        println!("\n{first_number} {operation_symbol} {second_number} = {result}\n");
    }
}

fn select_operation(stdin: &io::Stdin) -> String {
    println!("1- Addition\n2- Subtraction\n3- Multiplication\n4- Division\n5- Exit");
    let mut operation: String = String::new();
    stdin.read_line(&mut operation).expect("Failed to read");

    return operation;
}

fn get_operation_symbol(operation: u8) -> char {
    match operation {
        1 => '+',
        2 => '-',
        3 => '*',
        4 => '/',
        _ => '+',
    }
}

fn select_number(stdin: &io::Stdin, input_text: &str) -> String {
    println!("{}: ", input_text);
    let mut number: String = String::new();
    stdin.read_line(&mut number).expect("Failed to read");

    return number;
}

fn result(first_number: f64, second_number: f64, operation: u8) -> f64 {
    match operation {
        1 => first_number + second_number,
        2 => first_number - second_number,
        3 => first_number * second_number,
        4 => first_number / second_number,
        5 => exit(0),
        _ => first_number + second_number,
    }
}
