use eframe::egui;
use eframe::egui::Color32;

use crate::game::{Difficulty, GameState, LetterState};

pub struct WordSweeperApp {
    game_state: GameState,
    current_guess: String,
}

impl WordSweeperApp {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new_game(Difficulty::Easy),
            current_guess: String::new(),
        }
    }
}

impl eframe::App for WordSweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Easy").clicked() {
                    self.game_state = GameState::new_game(Difficulty::Easy);
                    self.current_guess.clear();
                }
                if ui.button("Medium").clicked() {
                    self.game_state = GameState::new_game(Difficulty::Medium);
                    self.current_guess.clear();
                }
                if ui.button("Hard").clicked() {
                    self.game_state = GameState::new_game(Difficulty::Hard);
                    self.current_guess.clear();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let grid_size = self.game_state.grid_size;

            egui::Grid::new("saper_grid")
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    for y in 0..grid_size {
                        for x in 0..grid_size {
                            let cell = &self.game_state.grid[y][x];
                            
                            let state = self.game_state.letter_states.get(&cell.letter)
                                .cloned()
                                .unwrap_or(LetterState::Neutral);

                            let color = match state {
                                LetterState::Neutral => Color32::from_rgb(50, 50, 50),
                                LetterState::Selected => Color32::from_rgb(50, 50, 200),
                                LetterState::Rejected => Color32::from_rgb(200, 50, 50),
                            };

                            let text = format!("{}\n({})", cell.letter, cell.adjacent_mines);
                            
                            let button = egui::Button::new(text)
                                .fill(color)
                                .min_size(egui::vec2(50.0, 50.0));

                            let response = ui.add(button);

                            if response.clicked() {
                                self.game_state.handle_letter_click(cell.letter, false);
                            } else if response.secondary_clicked() {
                                self.game_state.handle_letter_click(cell.letter, true);
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}