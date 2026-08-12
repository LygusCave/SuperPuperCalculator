use num_complex::Complex;
use std::fmt;
use std::f64::consts::PI;
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
        Operations::Roots => root(num1, num2),
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

fn root(n: f64, x: f64) -> Result<MathResult, &'static str> {
    if n == 0.0 || n.is_nan() || x.is_nan() || n.is_infinite()|| x.is_infinite() {
        return Err("Что-то очень страшное и странное");
    }

    // 1. Положительное основание или ноль
    if x >= 0.0 {
        return Ok(MathResult::Real(x.powf(1.0 / n)));
    }

    // 2. Отрицательное основание:
    // Проверка на целое нечетное n (как положительное, так и отрицательное)
    let is_integer = n.fract() == 0.0;
    let is_odd_int = is_integer && (n as i128).abs() % 2 != 0;

    if is_odd_int {
        // Для нечетного целого n: root(n, x) = -root(n, |x|)
        return Ok(MathResult::Real(-(-x).powf(1.0 / n)));
    }

    // 3. Комплексный главный корень для x < 0
    // (-|x|)^(1/n) = |x|^(1/n) * e^(i * pi / n)
    let r = (-x).powf(1.0 / n);
    let phi = PI / n;

    let mut re = r * phi.cos();
    let mut im = r * phi.sin();

    // Зачистка погрешностей плавающей точки относительно масштаба r
    let eps = f64::EPSILON * r.abs().max(1.0);
    if re.abs() < eps { re = 0.0; }
    if im.abs() < eps { im = 0.0; }

    if im == 0.0 {
        Ok(MathResult::Real(re))
    } else {
        Ok(MathResult::Complex(Complex::new(re, im)))
    }
}
