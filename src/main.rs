use eframe::egui;
use num_complex::Complex;
use std::{fmt, sync::Arc};
use calc::{to_superscript, to_subscript};
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "calculator",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(CalcApp::new(cc)))),
    )
}

#[derive(Debug)]
enum MathResult {
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
enum Operations {
    Add,
    Minus,
    Multiplication,
    Division,
    Roots,
    Logarithm,
}
impl Operations {
    fn symbol(&self) -> &'static str {
        match self {
            Operations::Add => "+",
            Operations::Minus => "-",
            Operations::Multiplication => "*",
            Operations::Division => "/",
            Operations::Roots => "√",
            Operations::Logarithm => "log",
        }
    }
    fn name(&self) -> &'static str {
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

fn calculate(num1: f64, num2: f64, opp: Operations) -> Result<MathResult, &'static str> {
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
                Ok(MathResult::Real(num2.log(num1)))
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
struct CalcApp {
    result_text: String,
    operation: Operations,
    num1: f64,
    num2: f64,
    history: String,
}

impl CalcApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        CalcApp {
            result_text: "0".to_string(),
            operation: Operations::Add,
            num1: 0.0,
            num2: 0.0,
            history: String::new(),
        }
    }
}
// Нейрослоп для шрифтов. Наверное надо будет руками переделать, хз
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Заворачиваем FontData в Arc или вызываем .into()
    fonts.font_data.insert(
        "custom_font".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/STIXTwoMath-Regular.ttf"
        ))),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "custom_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "custom_font".to_owned());

    ctx.set_fonts(fonts);
}
impl eframe::App for CalcApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.label("Результат:");
            ui.label(&self.result_text);
        });
        ui.separator();
        ui.label(r#"Это лучший калькулятор!1!!111"#);

        egui::ComboBox::from_label("Выберите операцию")
            .selected_text(self.operation.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.operation, Operations::Add, "Сложение");
                ui.selectable_value(&mut self.operation, Operations::Minus, "Вычитание");
                ui.selectable_value(&mut self.operation, Operations::Multiplication, "Сложение, но сложнее");
                ui.selectable_value(&mut self.operation, Operations::Division, "Деление");
                ui.selectable_value(&mut self.operation, Operations::Roots, "Вершки, да корешки");
                ui.selectable_value(&mut self.operation, Operations::Logarithm, "Логарифм");
            });
        ui.horizontal(|ui|{
            ui.add(egui::DragValue::new(&mut self.num1).speed(1.0).max_decimals(10));
            if self.operation == Operations::Logarithm { 
                ui.label("Основание");
            } else if self.operation == Operations::Roots {
                ui.label("Показатель");
            } else {
                ui.label("Число 1");
            }
            ui.add(egui::DragValue::new(&mut self.num2).speed(1.0).max_decimals(10));
                ui.label("Число 2");
            
        });
        if ui.button("Посчитать").clicked() {
            let mut new_line:String;
            let sym = self.operation.symbol();
            match calculate(self.num1, self.num2, self.operation) {
                    Ok(val) => self.result_text = val.to_string(),
                    Err(err) => self.result_text = err.to_string(),
            }
            match self.operation {
                Operations::Roots => {new_line = format!("{}{} {} = {}\n", to_superscript(self.num1.to_string()), sym, self.num2, self.result_text);}
                Operations::Logarithm => {new_line = format!("{}{} ({}) = {}\n", sym, to_subscript(self.num1.to_string()), self.num2, self.result_text);}
                _ => {new_line = format!("{} {} {} = {}\n", self.num1, sym, self.num2, self.result_text);}
                
            }
            self.history.insert_str(0, &new_line);     
        }
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                ui.label(&self.history);
        });
    }
}
