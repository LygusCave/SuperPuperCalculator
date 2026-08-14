use num_complex::Complex;
use calc::calc::{calculate, MathResult, Operations};
std::collections::VecDeque;
pub enum Token {
    Number(f64),
    //ComplexNumber(Complex<f64>), леново мне
    Op(char),
}
fn precedence(op: char) -> u8 {
    match op {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0,
    }
}

let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    let mut current_number = String::new();

    for ch in input.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            current_number.push(ch);
        } else if "+-*/".contains(ch) {
            // Если накопилось число — преобразуем его в Token::Number и добавляем в output
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    output.push(Token::Number(num));
                }
                current_number.clear();
            }

            // Учитываем приоритет операторов (алгоритм сортировочной станции)
            while let Some(&top) = operators.last() {
                if precedence(top) >= precedence(ch) {
                    output.push(Token::Op(operators.pop().unwrap()));
                } else {
                    break;
                }
            }
            operators.push(ch);
        }
    }

    // Добавляем последнее число, если оно есть
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<f64>() {
            output.push(Token::Number(num));
        }
    }

    // Выталкиваем оставшиеся операторы
    while let Some(op) = operators.pop() {
        output.push(Token::Op(op));
    }

    output
}
