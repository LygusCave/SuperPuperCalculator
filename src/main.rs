use eframe::egui;
use num_complex::Complex;
use std::{fmt, sync::Arc};
use calc::calc::{calculate, MathResult, Operations};
use calc::formatting::{to_subscript, to_superscript};
use calc::parser::{parse_and_calc};
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
    simple:String,
}

impl CalcApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        configure_custom_theme(&cc.egui_ctx);
        CalcApp {
            result_text: "0".to_string(),
            operation: Operations::Add,
            num1: 0.0,
            num2: 0.0,
            history: String::new(),
            simple: String::new(),
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
//нейрослоп для красоты
fn configure_custom_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // 1. Настройка акцентных цветов и фонов
    visuals.panel_fill = egui::Color32::from_rgb(33, 37, 41); 
    visuals.window_fill = egui::Color32::from_rgb(100, 200, 100); // Для выпадающих окон (ComboBox)
    visuals.faint_bg_color = egui::Color32::from_rgb(80, 180, 80);
    // 2. wigets
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 48, 71);
    // При наведении мыши (Hover)
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(67, 71, 106);
    // При нажатии (Active)
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(90, 95, 140);
    // 3. Толщина и цвет рамок (Borders)
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 63, 90));
    ctx.set_visuals(visuals);
}

impl eframe::App for CalcApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Сменить тему").clicked() {
                    if ui.visuals().dark_mode {
                        ui.ctx().set_visuals(egui::Visuals::light());
                    } else {
                        ui.ctx().set_visuals(egui::Visuals::dark());
                    }
                }
            });
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
                    ui.selectable_value(&mut self.operation, Operations::Power, "Степень");
                });
            ui.horizontal(|ui|{
                ui.add(egui::DragValue::new(&mut self.num1).speed(1.0).max_decimals(10));
                if self.operation == Operations::Logarithm || self.operation == Operations::Power { 
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
            ui.text_edit_singleline(&mut self.simple);
            if ui.button("Умная кнопка").clicked() {
                let result_text = match parse_and_calc(&self.simple) {
                    Ok(res) => res,
                    Err(err) => format!("Ошибка ({err})"),
                };
                let new_line = format!("{} = {}\n", &self.simple, result_text);
                self.history.insert_str(0, &new_line);
            }
        });
    }
}
