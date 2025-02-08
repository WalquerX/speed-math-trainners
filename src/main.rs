use eframe::egui;
mod game;
use crate::game::{Answer, Game, GameDifficulty, GameState, GameType, Problem};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Speed Math",
        native_options,
        Box::new(|_| Ok(Box::new(App::default()))),
    )
}
pub struct App {
    game: Game,
    current_answer: String,
    current_numerator: String,
    current_denominator: String,
}

impl Default for App {
    fn default() -> Self {
        let mut game = Game::new();
        let _ = game.generate_problem();
        Self {
            game,
            current_answer: "".to_string(),
            current_numerator: String::new(), // TODO: consider making it optional and use a struct for fraction
            current_denominator: String::new(),
        }
    }
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

            egui::menu::bar(ui, |ui| {
                // Game type selection
                ui.menu_button("Game Type", |ui| {
                    if ui.button("Addition").clicked() {
                        self.game.current_type = GameType::Addition;
                        self.game.generate_problem();
                    }
                    if ui.button("Subtraction").clicked() {
                        self.game.current_type = GameType::Subtraction;
                        self.game.generate_problem();
                    }
                    if ui.button("Multiplication").clicked() {
                        self.game.current_type = GameType::Multiplication;
                        self.game.generate_problem();
                    }
                    if ui.button("Division").clicked() {
                        self.game.current_type = GameType::Division;
                        self.game.generate_problem();
                    }
                    if ui.button("Fraction Addition").clicked() {
                        self.game.current_type = GameType::FractionAddition;
                        self.game.generate_problem();
                    }
                    if ui.button("Fraction Subtraction").clicked() {
                        self.game.current_type = GameType::FractionSubtraction;
                        self.game.generate_problem();
                    }
                });

                // Difficulty selection
                ui.menu_button("Difficulty", |ui| {
                    if ui.button("Basic").clicked() {
                        self.game.current_difficulty = GameDifficulty::Basic;
                        self.game.generate_problem();
                    }
                    if ui.button("Medium").clicked() {
                        self.game.current_difficulty = GameDifficulty::Medium;
                        self.game.generate_problem();
                    }
                    if ui.button("Hard").clicked() {
                        self.game.current_difficulty = GameDifficulty::Hard;
                        self.game.generate_problem();
                    }
                    if ui.button("Mastery").clicked() {
                        self.game.current_difficulty = GameDifficulty::Mastery;
                        self.game.generate_problem();
                    }
                    // Add other difficulties...
                });

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.heading("Game Levels");
            ui.add_space(10.0);

            let game_types = [
                ("Addition", GameType::Addition),
                ("Subtraction", GameType::Subtraction),
                ("Multiplication", GameType::Multiplication),
                ("Division", GameType::Division),
                ("Fraction Addition", GameType::FractionAddition),
                ("Fraction Subtraction", GameType::FractionSubtraction),
            ];

            let difficulties = [
                ("Basic", GameDifficulty::Basic, 100), // Score threshold for golden stars
                ("Medium", GameDifficulty::Medium, 200),
                ("Hard", GameDifficulty::Hard, 300),
                ("Mastery", GameDifficulty::Mastery, 400),
            ];

            for (game_name, game_type) in game_types.iter() {
                ui.collapsing(*game_name, |ui| {
                    for (difficulty_name, difficulty, threshold) in difficulties.iter() {
                        ui.horizontal(|ui| {
                            if ui.button(*difficulty_name).clicked() {
                                self.game.current_type = *game_type;
                                self.game.current_difficulty = *difficulty;
                                self.game.generate_problem();
                            }

                            // Add space between difficulty name and stars
                            ui.add_space(10.0);

                            // Draw 10 stars
                            //let score = self.session.get_score(game_type, difficulty); // You'll need to implement this
                            let score = 10u32;
                            for _ in 0..10 {
                                let star_color = if score >= *threshold {
                                    egui::Color32::GOLD
                                } else {
                                    egui::Color32::GRAY
                                };

                                ui.label(egui::RichText::new("★").color(star_color).size(16.0));
                            }
                        });
                    }
                });
                ui.add_space(5.0);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Speed Math");

            ui.horizontal(|ui| {
                ui.label("Score: ");
                ui.label(self.game.score.to_string());
            });

            ui.add_space(20.0);

            if let Some(problem) = &self.game.current_problem {
                ui.heading(&problem.problem);

                match self.game.game_state {
                    GameState::NotStarted => {
                        if ui.button("Start Round").clicked() {
                            self.game.generate_problem();
                            self.game.game_state = GameState::Playing;
                        }
                    }
                    GameState::Playing => {
                        match self.game.current_type {
                            GameType::Addition
                            | GameType::Subtraction
                            | GameType::Multiplication
                            | GameType::Division => {
                                let response = ui.text_edit_singleline(&mut self.current_answer);

                                for _ in 0..self.game.problems_per_round {
                                    if response.changed() {
                                        if let Ok(user_answer) = self.current_answer.parse::<i32>()
                                        {
                                            if self.game.check_answer(&Answer::Integer(user_answer))
                                            {
                                                self.current_answer.clear();

                                                if self.game.is_round_completed() {
                                                    self.game.game_state =
                                                        GameState::RoundComplete;
                                                } else {
                                                    self.game.generate_problem();
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            GameType::FractionAddition | GameType::FractionSubtraction => {
                                let mut should_check = false;

                                ui.horizontal(|ui| {
                                    let numerator_response =
                                        ui.text_edit_singleline(&mut self.current_numerator);

                                    ui.label("/");

                                    let denominator_response =
                                        ui.text_edit_singleline(&mut self.current_denominator);

                                    should_check = numerator_response.changed()
                                        || denominator_response.changed();
                                });

                                if should_check {
                                    if let (Ok(num), Ok(den)) = (
                                        self.current_numerator.parse::<i32>(),
                                        self.current_denominator.parse::<i32>(),
                                    ) {
                                        if den != 0 {
                                            // Avoid division by zero

                                            let user_answer = Answer::Fraction {
                                                numerator: num,

                                                denominator: den,
                                            };

                                            if self.game.check_answer(&user_answer) {
                                                self.current_numerator.clear();
                                                self.current_denominator.clear();
                                                self.game.generate_problem();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    GameState::RoundComplete => {
                        ui.heading("Round Completed!");
                        ui.label(format!("Final Score: {}", self.game.score));

                        if ui.button("Play Again").clicked() {
                            // Reset the session for a new round
                            self.game.score = 0;
                            self.game.current_round_completed_problems = 0;
                            self.game.game_state = GameState::NotStarted;
                        }
                    }
                }
            } else {
                self.game.generate_problem();
            }
        });
    }
}

// test
// generate game
// answer
// check answer
// update score

// create a new game selecting the type of game and its difficulty. The game type can be addition,
// subtraction, multiplication, or division. The difficulty can be basics, medium, hard or mastery.
// for addition basics, the numbers are between 1 and 9 inclusive.
// for addition medium, the numbers are between 10 and 99 inclusive.
// for addition hard, the numbers are between 100 and 999 inclusive.
// for addition mastery, the numbers are between 1000 and 9999 inclusive.
