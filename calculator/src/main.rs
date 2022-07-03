use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush"); // show value ini terminal
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Selamat datang di calculator_programer");
    println!("--------------------");
    loop {

        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        print!("first number?: ");
        read(&mut num1);

        print!("second number?:");
        read(&mut num2);

        print!("What operation would you like to do? [+-*/%]:");
        read(&mut operator);

        println!("{},{},{}", num1, num2, operator);

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let operators = String::from("+-*/%");

        if !operators.contains(operator) {
            println!("operator {} not found in list", operator);
            continue;
        }

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            '%' => num2 * num1 /100.0,
            _ => panic!("Invalid operator {}", operator),
        };

        println!("the result {} {} {} = {}", num1, operator, num2, result);
    }
}
