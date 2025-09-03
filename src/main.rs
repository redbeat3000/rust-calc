use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Op(char),
    LParen,
    RParen,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch.is_digit(10) || ch == '.' {
            let mut num = String::new();
            while let Some(&c2) = chars.peek() {
                if c2.is_digit(10) || c2 == '.' {
                    num.push(c2);
                    chars.next();
                } else {
                    break;
                }
            }
            match num.parse::<f64>() {
                Ok(n) => tokens.push(Token::Number(n)),
                Err(_) => return Err(format!("Invalid number: {}", num)),
            }
            continue;
        }
        match ch {
            '+' | '-' | '*' | '/' | '^' | '%' => {
                tokens.push(Token::Op(ch));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            _ => return Err(format!("Invalid character: '{}'", ch)),
        }
    }

    // Handle unary minus: convert unary '-' to a '0' then '-'
    let mut fixed = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if let Token::Op('-') = tokens[i] {
            let is_unary = if i == 0 {
                true
            } else {
                matches!(tokens[i - 1], Token::Op(_) | Token::LParen)
            };
            if is_unary {
                fixed.push(Token::Number(0.0));
                fixed.push(Token::Op('-'));
                i += 1;
                continue;
            }
        }
        fixed.push(tokens[i].clone());
        i += 1;
    }

    Ok(fixed)
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn is_right_associative(op: char) -> bool {
    matches!(op, '^')
}

fn shunting_yard(tokens: &[Token]) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut ops: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token.clone()),
            Token::Op(op1) => {
                while let Some(top) = ops.last() {
                    match top {
                        Token::Op(op2) => {
                            let p1 = precedence(*op1);
                            let p2 = precedence(*op2);
                            if (p1 < p2) || (p1 == p2 && !is_right_associative(*op1)) {
                                output.push(ops.pop().unwrap());
                                continue;
                            }
                        }
                        _ => {}
                    }
                    break;
                }
                ops.push(token.clone());
            }
            Token::LParen => ops.push(token.clone()),
            Token::RParen => {
                while let Some(t) = ops.pop() {
                    if matches!(t, Token::LParen) {
                        break;
                    } else {
                        output.push(t);
                    }
                }
            }
        }
    }

    while let Some(t) = ops.pop() {
        if matches!(t, Token::LParen | Token::RParen) {
            return Err("Mismatched parentheses".into());
        }
        output.push(t);
    }

    Ok(output)
}

fn eval_rpn(tokens: &[Token]) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Op(op) => {
                if *op == '%' {
                    if let Some(a) = stack.pop() {
                        stack.push(a / 100.0);
                    } else {
                        return Err("Not enough operands for %".into());
                    }
                    continue;
                }

                let b = stack.pop().ok_or("Not enough operands")?;
                let a = stack.pop().ok_or("Not enough operands")?;
                let res = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        if b == 0.0 {
                            return Err("Division by zero".into());
                        }
                        a / b
                    }
                    '^' => a.powf(b),
                    _ => return Err(format!("Unknown operator: {}", op)),
                };
                stack.push(res);
            }
            _ => return Err("Invalid token in RPN".into()),
        }
    }

    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Invalid expression".into())
    }
}

fn evaluate(expr: &str) -> Result<f64, String> {
    let tokens = tokenize(expr)?;
    let rpn = shunting_yard(&tokens)?;
    eval_rpn(&rpn)
}

fn print_help() {
    println!("Rust Calculator REPL");
    println!("Type expressions, e.g.: 2 + 3 * (4 - 1) ^ 2");
    println!("Operators: + - * / ^ % (percent converts number to fraction, e.g. 50% -> 0.5)");
    println!("Commands: quit, exit, help, clear");
}

fn main() {
    let mut input = String::new();
    print_help();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Input error, try again.");
            continue;
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }
        match line.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Goodbye.");
                break;
            }
            "help" => {
                print_help();
                continue;
            }
            "clear" => {
                // Not portable, but try to clear the screen
                print!("\x1B[2J\x1B[1;1H");
                continue;
            }
            _ => {}
        }

        match evaluate(line) {
            Ok(result) => {
                // Trim trailing .0 for whole numbers
                if (result.fract()).abs() < 1e-12 {
                    println!("{}", result.trunc() as i64);
                } else {
                    println!("{}", result);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
