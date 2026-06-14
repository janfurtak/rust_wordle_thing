use eframe::egui;
use eframe::egui::Color32;

use crate::game::{guess_word, Difficulty, GameState, LetterFeedback, LetterState};

pub struct WordSweeperApp {
    game_state: GameState,
    current_guess: String,
    past_guesses: Vec<String>,
}
const NUM_OF_GUESSES: i8 = 3;

impl WordSweeperApp {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new_game(Difficulty::Easy),
            current_guess: String::new(),
            past_guesses: Vec::new(),
        }
    }
}

impl eframe::App for WordSweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Obsługa klawiatury
        let target_len = self.game_state.target_word.len();

        // Obsługa Backspace
        if ctx.input(|i| i.key_pressed(egui::Key::Backspace)) {
            self.current_guess.pop();
        }

        // Obsługa wpisywania liter
        for event in ctx.input(|i| i.events.clone()) {
            if let egui::Event::Text(text) = event {
                for c in text.chars() {
                    if c.is_ascii_alphabetic() && self.current_guess.len() < target_len {
                        self.current_guess.push(c.to_ascii_uppercase());
                    }
                }
            }
        }

        let submit_pressed_keyboard = ctx.input(|i| i.key_pressed(egui::Key::Enter));
        let mut do_submit = false;

        // Stałe dla rzędów liter — wspólne dla historii, inputa i wybranych liter
        let letter_size = 40.0;
        let letter_spacing = 5.0;
        let submit_btn_width = 80.0;
        let submit_btn_gap = 15.0;

        // Szerokość rzędu z literami (bez przycisku Submit)
        let letters_row_width = target_len as f32 * letter_size
            + (target_len.saturating_sub(1)) as f32 * letter_spacing;

        // Szerokość rzędu inputa (litery + Submit)
        let input_row_width = letters_row_width + submit_btn_gap + submit_btn_width;

        // Panel z przyciskami wyboru trudności
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(15.0);

                let difficulties = [
                    ("Easy", Difficulty::Easy),
                    ("Medium", Difficulty::Medium),
                    ("Hard", Difficulty::Hard),
                ];

                for (label, diff) in difficulties {
                    let is_active = self.game_state.difficulty == diff;

                    let button = egui::Button::new(label)
                        .fill(if is_active {
                            Color32::from_rgb(50, 150, 50)
                        } else {
                            Color32::TRANSPARENT
                        });

                    if ui.add(button).clicked() {
                        self.game_state = GameState::new_game(diff);
                        self.current_guess.clear();
                        self.past_guesses.clear();
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(10.0);

                        let grid_size = self.game_state.grid_size;

                        let cell_size: f32 = match self.game_state.difficulty {
                            Difficulty::Easy => 50.0,
                            Difficulty::Medium => 45.0,
                            Difficulty::Hard => 38.0,
                        };
                        let spacing = 10.0;
                        let grid_width = grid_size as f32 * cell_size
                            + (grid_size - 1) as f32 * spacing;

                        // Grid
                        ui.allocate_ui_with_layout(
                            egui::vec2(grid_width, grid_width),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                egui::Grid::new("saper_grid")
                                    .spacing([spacing, spacing])
                                    .show(ui, |ui| {
                                        for y in 0..grid_size {
                                            for x in 0..grid_size {
                                                let cell = &self.game_state.grid[y][x];

                                                let state = self.game_state.letter_states
                                                    .get(&cell.letter)
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
                                                    .min_size(egui::vec2(cell_size, cell_size));

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
                            },
                        );

                        ui.add_space(10.0);

                        // Przycisk "Clear grid"
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.add_space(40.0);
                            if ui.button("Clear grid").clicked() {
                                for state in self.game_state.letter_states.values_mut() {
                                    *state = LetterState::Neutral;
                                }
                            }
                        });

                        ui.add_space(20.0);

                        // Historia guessów — każdy rząd wycentrowany przez allocate_ui_with_layout
                        for guess in &self.past_guesses {
                            let feedback = guess_word(guess, &self.game_state.target_word);

                            ui.allocate_ui_with_layout(
                                egui::vec2(letters_row_width, letter_size),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    for (g, f) in guess.chars().zip(feedback.iter()) {
                                        let color = match f {
                                            LetterFeedback::Correct   => Color32::from_rgb(50, 150, 50),
                                            LetterFeedback::Misplaced => Color32::from_rgb(200, 150, 20),
                                            LetterFeedback::Absent    => Color32::from_rgb(80, 80, 80),
                                        };

                                        let button = egui::Button::new(
                                            egui::RichText::new(g.to_string()).color(Color32::WHITE)
                                        )
                                        .fill(color)
                                        .sense(egui::Sense::hover());

                                        ui.add_sized(egui::vec2(letter_size, letter_size), button);
                                        ui.add_space(letter_spacing);
                                    }
                                },
                            );
                            ui.add_space(5.0);
                        }

                        // Rząd wpisywania hasła — szerszy bo zawiera przycisk Submit
                        ui.allocate_ui_with_layout(
                            egui::vec2(input_row_width, letter_size),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                let current_chars: Vec<char> = self.current_guess.chars().collect();

                                for i in 0..target_len {
                                    let display_char = if i < current_chars.len() {
                                        current_chars[i].to_string()
                                    } else {
                                        "_".to_string()
                                    };

                                    let button = egui::Button::new(
                                        egui::RichText::new(display_char).color(Color32::WHITE)
                                    )
                                    .fill(Color32::from_rgb(40, 40, 40))
                                    .sense(egui::Sense::hover());

                                    ui.add_sized(egui::vec2(letter_size, letter_size), button);
                                    ui.add_space(letter_spacing);
                                }

                                ui.add_space(submit_btn_gap);
                                let submit_btn = ui.add_sized(
                                    egui::vec2(submit_btn_width, letter_size),
                                    egui::Button::new("Submit"),
                                );

                                if submit_btn.clicked() || submit_pressed_keyboard {
                                    do_submit = true;
                                }
                            },
                        );

                        if do_submit && self.current_guess.len() == target_len {
                            self.past_guesses.push(self.current_guess.clone());
                            self.current_guess.clear();
                        }

                        ui.add_space(40.0);

                        // Wybrane litery z siatki
                        let mut selected_letters: Vec<char> = self.game_state.letter_states.iter()
                            .filter(|(_, state)| **state == LetterState::Selected)
                            .map(|(letter, _)| *letter)
                            .collect();

                        selected_letters.sort();

                        let n = selected_letters.len() as f32;
                        let selected_row_width = if n > 0.0 {
                            n * letter_size + (n - 1.0) * letter_spacing
                        } else {
                            0.0
                        };

                        ui.allocate_ui_with_layout(
                            egui::vec2(selected_row_width, letter_size),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                let frame_color = Color32::from_rgb(50, 50, 200);
                                for letter in selected_letters {
                                    let button = egui::Button::new(
                                        egui::RichText::new(letter.to_string()).color(Color32::WHITE)
                                    )
                                    .fill(frame_color)
                                    .sense(egui::Sense::hover());

                                    ui.add_sized(egui::vec2(letter_size, letter_size), button);
                                    ui.add_space(letter_spacing);
                                }
                            },
                        );

                        ui.add_space(20.0);
                    });
                });
        });
    }
}