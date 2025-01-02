use eframe::egui;
use rand::Rng;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Speed Math", native_options, Box::new(|_| Ok(Box::new(App::default()))))
}

pub struct App {
    problem: String,
    answer: String,
    correct_answer: i32,
    result: Option<String>,
    score: i32,
}

impl Default for App {
    fn default() -> Self {
        let (problem, answer) = generate_problem();
        Self {
            problem,
            answer: String::new(),
            correct_answer: answer,
            result: None,
            score: 0,
        }
    }
}

fn generate_problem() -> (String, i32) {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=10);
    let b = rng.gen_range(1..=10);
    (format!("{} + {}", a, b), a + b)
}

impl App {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Speed Math");

            ui.horizontal(|ui| {
                ui.label("Score: ");
                ui.label(self.score.to_string());
            });

            ui.add_space(20.0);
            ui.heading(&self.problem);
            let response = ui.text_edit_singleline(&mut self.answer);
            
            if response.changed() {
                if let Ok(user_answer) = self.answer.parse::<i32>() {
                    if user_answer == self.correct_answer {
                        self.score += 1;
                        let (new_problem, new_answer) = generate_problem();
                        self.problem = new_problem;
                        self.correct_answer = new_answer;
                        self.answer.clear();
                    }
            }}
        });
    }
}