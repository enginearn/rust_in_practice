use std::io;
use regex::Regex;

fn main() {
    println!("Welcome to the Calculator!");
    loop {
        println!("Please input your expression (e.g. 2 + 2):");
        let mut expression = String::new();
        io::stdin().read_line(&mut expression).expect("Failed to read line");
        let expression = expression.trim();
        if expression == "exit" {
            println!("Goodbye!");
            break;
        }
        let result = calculate(expression);
        println!("Result: {}", result);
    }
}

fn calculate(expression: &str) -> f64 {
    let re = Regex::new(r"([\d.]+)\s*(\+|\-|\*|/)\s*([\d.]+)").unwrap();
    let captures = re.captures(expression).unwrap();
    let left = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
    let operator = captures.get(2).unwrap().as_str();
    let right = captures.get(3).unwrap().as_str().parse::<f64>().unwrap();
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => panic!("Unknown operator: {}", operator),
    }
}
