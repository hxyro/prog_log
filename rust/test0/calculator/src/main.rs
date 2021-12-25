#![allow(unused_variables, unused_mut)]
use std::io::{stdin, stdout, Write};

fn main() {
    println!("welcome to the calculator:");
    println!("===========================");

    print!("first number: ");
    let num1: f32 = read().trim().parse().unwrap();
    print!("second number: ");
    let num2: f32 = read().trim().parse().unwrap();
    print!("operator: ");
    let operator: char = read().trim().chars().next().unwrap();
    
    let operators = String::from("*+-/");
    if !operators.contains(operator){
        println!("unknown operator");
        return;
    }
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '/' => num1 / num2,
        '*' => num1 * num2,
         _  => panic!("eh")
    };

    println!("{} {} {} = {}", num1, operator, num2, result);
}

fn read() -> String
{
    let mut input: String = String::new();
    stdout().flush().expect("failed to flush");
    stdin().read_line(&mut input).expect("failed to flush");
    input.to_string()
}
