/**
 * Very Simple Calculator
 *
 * Prompts user to write an expression in the form;
 *
 * a <op> b
 * 
 * Prints the result
 */

use std::io;
use std::io::Write;

fn calculate(lhs: &str, operator: &str, rhs: &str) -> f32 {
    let l: f32 = lhs.trim().parse().unwrap();
    let r: f32 = rhs.trim().parse().unwrap();
    return match operator.trim() {
        "+" => l + r,
        "-" => l - r,
        "/" => l / r,
        "x" => l * r,
        _   => panic!("Invalid operator!")
    }
}

enum State {
    Continue,
    Usage,
    Quit
}

fn prompt() -> State {
    
    print!(">>> ");

    std::io::stdout().flush().unwrap();

    let mut expression = String::new();


    io::stdin().read_line(&mut expression).expect("Failed to read line");

    if expression.trim() == "q" {
        return State::Quit 
    }

    let expr_values: Vec<&str> = expression.split(" ").collect();

    if expr_values.len() != 3 {
        return State::Usage
    }
    
    let lhs = expr_values[0].trim();
    let op = expr_values[1].trim();
    let rhs = expr_values[2].trim();

    let result = calculate(lhs, op, rhs);

    println!("{lhs} {op} {rhs} = {result}");

    return State::Continue;
}

fn main() {

    let usage_message = "USAGE: LHS Operator RHS";
    
    println!("[Usage: <a> <+|-|/|x> <b> e.g. (>>> 2 + 3) ; Quit: q");

    loop {
        match prompt() {
            State::Usage => println!("{usage_message}"),
            State::Quit => break,
            State::Continue => (),

        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_calculate() {
        assert_eq!(calculate("3", "+", "3"), 6.0);
        assert_eq!(calculate("4", "x", "3"), 12.0);
        assert_eq!(calculate("12", "/", "4"), 3.0);
        assert_eq!(calculate("50", "-", "30"), 20.0);
    }

}
