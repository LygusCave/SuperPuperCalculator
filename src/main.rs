use eframe::egui;
use std::fmt::Write;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "calculator",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(CalcApp::new(cc)))),
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operations {
    Add,
    Minus,
    Multiplication,
    Division,
}
impl Operations {
    fn symbol(&self) -> &'static str {
        match self {
            Operations::Add => "+",
            Operations::Minus => "-",
            Operations::Multiplication => "*",
            Operations::Division => "/",
        }
    }
}

fn calculate(num1: f64, num2: f64, opp: Operations) -> Result<f64, &'static str> {
    match opp {
        Operations::Add => Ok(num1 + num2),
        Operations::Minus => Ok(num1 - num2),
        Operations::Multiplication => Ok(num1 * num2),
        Operations::Division => {
            if num2 == 0.0 {
                Err("Не дели на ноль!")
            } else {
                Ok(num1 / num2)
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
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        CalcApp {
            result_text: "0".to_string(),
            operation: Operations::Add,
            num1: 0.0,
            num2: 0.0,
            history: String::new(),
        }
    }
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
            .selected_text(match self.operation {
                Operations::Add => "Сложение",
                Operations::Minus => "Вычитание",
                Operations::Multiplication => "Сложение, но сложнее",
                Operations::Division => "Деление",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.operation, Operations::Add, "Сложение");
                ui.selectable_value(&mut self.operation, Operations::Minus, "Вычитание");
                ui.selectable_value(&mut self.operation, Operations::Multiplication, "Сложение, но сложнее");
                ui.selectable_value(&mut self.operation, Operations::Division, "Деление");
            });

        ui.add(egui::DragValue::new(&mut self.num1).speed(1.0));
        ui.add(egui::DragValue::new(&mut self.num2).speed(1.0));

        if ui.button("Посчитать").clicked() {
            match calculate(self.num1, self.num2, self.operation) {
                Ok(val) => self.result_text = val.to_string(),
                Err(err) => self.result_text = err.to_string(),
            }
            let sym = self.operation.symbol();
            let new_line = format!("{} {} {} = {}\n", self.num1, sym, self.num2, self.result_text);
            self.history.insert_str(0, &new_line);     
        }
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                ui.label(&self.history);
        });
    }
}
