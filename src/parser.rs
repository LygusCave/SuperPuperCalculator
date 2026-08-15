use gyard::{InputToken, op::Math};
// Я дурачок, которому лень делать самому
fn tokenize(input: &str) -> Result<Vec<InputToken>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Игнорируем пробелы
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Числа (считываем всю цепочку цифр)
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
                tokens.push(InputToken::Value(val));
            }
            // Математические операторы
            '+' => {
                tokens.push(InputToken::Operator(Math::Add));
                chars.next();
            }
            '*' => {
                tokens.push(InputToken::Operator(Math::Mul));
                chars.next();
            }
            '-' => {
                tokens.push(InputToken::Operator(Math::Sub));
                chars.next();
            }
            '/' => {
                tokens.push(InputToken::Operator(Math::Div));
                chars.next();
            }
            // Скобки
            '(' => {
                tokens.push(InputToken::LeftParen);
                chars.next();
            }
            ')' => 
                tokens.push(InputToken::RightParen);
                chars.next();
            }
            // Имена функций или переменных (например, "sin", "cos")
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
                // Для макроса/кода, если функции хранятся как &'static str или String
                // В зависимости от того, как определен InputToken в gyard:
                tokens.push(InputToken::Function(Box::leak(name.into_boxed_str())));
            }
            _ => return Err(format!("Unexpected character: {ch}")),
        }
    }

    Ok(tokens)
}

pub fn parser(input: &str) -> Result<String, &'static str> {
    match tokenize(input) {
        Ok(infix) => Ok(gyard::to_postfix(&infix)),
        Err(err) => Err(err),
    }
}
