use eframe::egui;
use num_complex::Complex;
use std::{fmt, sync::Arc};
use calc::calc::{calculate, MathResult, Operations};
use calc::formatting::{to_subscript, to_superscript};
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "calculator",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(CalcApp::new(cc)))),
    )
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
