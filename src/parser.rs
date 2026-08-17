use crate::calc::{calculate, MathResult, Operations};
use gyard::{op::Math, InputToken, OutputToken};

type MyInput = InputToken<i32, &'static str, Math>;
type MyOutput = OutputToken<i32, &'static str, Math>;

fn tokenize(input: &str) -> Result<Vec<MyInput>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Игнорируем пробелы
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Числа
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        num_str.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let val: i32 = num_str.parse().map_err(|e| format!("Invalid number: {e}"))?;
                tokens.push(MyInput::Value(val));
            }
            // Математические операторы
            '+' => {
                tokens.push(MyInput::Operator(Math::Add));
                chars.next();
            }
            '*' => {
                tokens.push(MyInput::Operator(Math::Mul));
                chars.next();
            }
            '-' => {
                // Проверяем, является ли минус унарным
                let is_unary = match tokens.last() {
                    None => true, // Минус в начале строки: "-5"
                    Some(MyInput::Operator(_)) | Some(MyInput::LeftParen) => true, // После оператора или '(': "(-89)"
                    _ => false,
                };

                if is_unary {
                    // Превращаем "-X" в "0 - X"
                    tokens.push(MyInput::Value(0));
                }

                tokens.push(MyInput::Operator(Math::Sub));
                chars.next();
            }
            '/' => {
                tokens.push(MyInput::Operator(Math::Div));
                chars.next();
            }
            // Скобки
            '(' => {
                tokens.push(MyInput::LeftParen);
                chars.next();
            }
            ')' => { // <- Была пропущена открывающая скобка блока
                tokens.push(MyInput::RightParen);
                chars.next();
            }
            // Имена функций или переменных
            'a'..='z' | 'A'..='Z' => {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(MyInput::Function(Box::leak(name.into_boxed_str())));
            }
            _ => return Err(format!("Unexpected character: {ch}")),
        }
    }

    Ok(tokens)
}

pub fn parse_and_calc(input: &str) -> Result<String, String> {
    let tokens = tokenize(input)?;
        let rpn_tokens: Vec<MyOutput> = gyard::to_postfix(tokens)
        .map_err(|e| format!("Ошибка в скобках: {e:?}"))?;
    let mut stack: Vec<f64> = Vec::new();

    for token in rpn_tokens {
        match token {
            OutputToken::Value(val) => stack.push(val as f64),
            OutputToken::Operator(op) => {
                let right = stack.pop().ok_or("Ошибка в выражении: не хватает операнда")?;
                let left = stack.pop().ok_or("Ошибка в выражении: не хватает операнда")?;

                let calc_op = match op {
                    Math::Add => Operations::Add,
                    Math::Sub => Operations::Minus,
                    Math::Mul => Operations::Multiplication,
                    Math::Div => Operations::Division,
                    Math::Exponent => return Err("Операция не поддерживается".to_string()),
                    _ => return Err("мне лень".to_string()),
                };

                let math_res = calculate(left, right, calc_op)?;

                let val = match math_res {
                    MathResult::Real(v) => v,
                    MathResult::Complex(_) => return Err("Комплексные числа не поддерживаются".to_string()),
                };

                stack.push(val);
            }
            _ => return Err("Неподдерживаемый токен в ОПЗ".to_string()),
        }
    }

    let final_result = stack.pop().ok_or("Пустое выражение")?;
    Ok(final_result.to_string())
}
