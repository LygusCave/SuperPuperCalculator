use num_complex::Complex;
use std::fmt;
#[derive(Debug)]
pub enum MathResult {
    Real(f64),
    Complex(Complex<f64>),
}
impl fmt::Display for MathResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathResult::Real(val) => write!(f, "{}", val),
            MathResult::Complex(c) => {
                if c.im < 0.0 {write!(f, "{} - {}i", c.re, c.im.abs())}
                else {write!(f, "{} + {}i", c.re, c.im)}
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operations {
    Add,
    Minus,
    Multiplication,
    Division,
    Roots,
    Logarithm,
}
impl Operations {
    pub fn symbol(&self) -> &'static str {
        match self {
            Operations::Add => "+",
            Operations::Minus => "-",
            Operations::Multiplication => "*",
            Operations::Division => "/",
            Operations::Roots => "√",
            Operations::Logarithm => "log",
        }
    }
    pub fn name(&self) -> &'static str {
        match self {    
            Operations::Add => "Сложение",
            Operations::Minus => "Вычитание",
            Operations::Multiplication => "Сложение, но сложнее",
            Operations::Division => "Деление",
            Operations::Roots => "Вершки, да корешки",
            Operations::Logarithm => "Логарифм"

        }
    }
}

pub fn calculate(num1: f64, num2: f64, opp: Operations) -> Result<MathResult, &'static str> {
    match opp {
        Operations::Add => Ok(MathResult::Real(num1 + num2)),
        Operations::Minus => Ok(MathResult::Real(num1 - num2)),
        Operations::Multiplication => Ok(MathResult::Real(num1 * num2)),
        Operations::Division => {
            if num2.abs() < f64::EPSILON {
                Err("Не дели на ноль!")
            } else {
                Ok(MathResult::Real(num1 / num2))
            }
        }
        Operations::Roots => {
            if num1.abs() < f64::EPSILON {
                return Err("Чтоб на нули не делить далее");
            }

            if num2 >= 0.0 {
                Ok(MathResult::Real(num2.powf(1.0 / num1)))
            } else {
            if (num1 - num1.round()).abs() > 1e-7 {
                Err("Просто, зачем? Тебе заняться нечем? Иди траву потрогай, хз. Никаких дробных степеней сейчас, прошу, я не вечный")
            } else {
                let n = num1.round() as i64;
                let abs_n = n.abs() as f64;

                if n % 2 == 0 {
                    let magnitude = (-num2).powf(1.0 / abs_n);
                    let angle = std::f64::consts::PI / abs_n;
                
                    let mut re = magnitude * angle.cos();
                    let mut im = magnitude * angle.sin();

                    if n < 0 {
                        let norm_sq = re * re + im * im;
                        re /= norm_sq;
                        im = -im / norm_sq;
                    }

                    if re.abs() < 1e-12 { re = 0.0; }
                        Ok(MathResult::Complex(Complex::new(re, im)))
                    } else {
                        let root = (-num2).powf(1.0 / abs_n);
                        let result = if n > 0 { -root } else { -1.0 / root };
                        Ok(MathResult::Real(result))
                    }
                }
            }
        }
        Operations::Logarithm => {
            if num1 <= 0.0 || (num1 - 1.0).abs() < f64::EPSILON || num1.is_nan() {
                return Err("Я ещё не на столько с ума сошёл, чтобы эту бурду делать");
            }
            if num2 > 0.0 {
                let mut res = num2.log(num1);
                if (res - res.round()).abs() < 1e-12 {
                    res = res.round();
                }
                Ok(MathResult::Real(res))
            } else if num2 < 0.0 {
                let ln_base = num1.ln();
                let re = (-num2).ln() / ln_base;
                let im = std::f64::consts::PI / ln_base;
                Ok(MathResult::Complex(Complex::new(re, im)))    
            } else if num2.is_nan(){
                Err("Логарифм от NaN это что-то из области Javascript")
            } else {
                Err("Я ещё не придумал логарифм нуля. Если знаешь решение - пиши в issues, вдруг ты гений.")
            }
        }
    }
}

