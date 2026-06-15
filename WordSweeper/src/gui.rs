use eframe::egui;
use eframe::egui::Color32;

use crate::game::{guess_word, Difficulty, GameState, LetterFeedback, LetterState};

pub struct WordSweeperApp {
    game_state: GameState,
    current_guess: String,
    past_guesses: Vec<String>,
    final_time: Option<std::time::Duration>,
}
const NUM_OF_GUESSES: i8 = 3;

impl WordSweeperApp {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new_game(Difficulty::Easy),
            current_guess: String::new(),
            past_guesses: Vec::new(),
            final_time: None,
        }
    }
}

impl eframe::App for WordSweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut visuals = ctx.style().visuals.clone();
        visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
        visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
        visuals.widgets.active.rounding = egui::Rounding::same(6.0);
        visuals.widgets.open.rounding = egui::Rounding::same(6.0);

        visuals.panel_fill = Color32::from_rgb(20, 20, 20); 
        ctx.set_visuals(visuals);

        let target_len = self.game_state.target_word.len();

        let won = self.past_guesses.last().map(|g| g == &self.game_state.target_word).unwrap_or(false);
        let lost = !won && self.past_guesses.len() >= NUM_OF_GUESSES as usize;
        let game_over = won || lost;

        if game_over && self.final_time.is_none() {
            self.final_time = Some(self.game_state.start_time.elapsed());
        }

        if !game_over {
            ctx.request_repaint();
        }

        let elapsed_duration = self.final_time.unwrap_or_else(|| self.game_state.start_time.elapsed());
        let minutes = elapsed_duration.as_secs() / 60;
        let seconds = elapsed_duration.as_secs() % 60;
        let time_string = format!("{:02}:{:02}", minutes, seconds);

        if !game_over {
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
        }

        let submit_pressed_keyboard = !game_over && ctx.input(|i| i.key_pressed(egui::Key::Enter));
        let mut do_submit = false;

        let letter_size = 40.0;
        let letter_spacing = 5.0;
        let submit_btn_width = 80.0;
        let submit_btn_gap = 15.0;
        let backspace_btn_width = 40.0;

        let letters_row_width = target_len as f32 * letter_size
            + (target_len.saturating_sub(1)) as f32 * letter_spacing;

        let input_row_width = letters_row_width + submit_btn_gap + backspace_btn_width + letter_spacing + submit_btn_width;

        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none().fill(Color32::from_rgb(26, 26, 26)).inner_margin(10.0))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(15.0);

                    let difficulties = [
                        ("Easy", Difficulty::Easy),
                        ("Medium", Difficulty::Medium),
                        ("Hard", Difficulty::Hard),
                    ];

                    for (label, diff) in difficulties {
                        let is_active = self.game_state.difficulty == diff;

                        let button = egui::Button::new(
                            egui::RichText::new(label)
                                .size(13.0)
                                .color(if is_active { Color32::WHITE } else { Color32::LIGHT_GRAY })
                        )
                        .fill(if is_active {
                            Color32::from_rgb(106, 170, 100)
                        } else {
                            Color32::from_rgb(40, 40, 40)
                        });

                        if ui.add(button).clicked() {
                            self.game_state = GameState::new_game(diff);
                            self.current_guess.clear();
                            self.past_guesses.clear();
                            self.final_time = None;
                        }
                        ui.add_space(5.0);
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Time: {}", time_string)).size(18.0));
                ui.add_space(20.0);
                ui.label(egui::RichText::new(format!("Attempts: {}/{}", self.past_guesses.len(), NUM_OF_GUESSES)).size(16.0));
            });
            ui.add_space(10.0);

            ui.add_enabled_ui(!game_over, |ui| {
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

                                                    let (rect, response) = ui.allocate_exact_size(
                                                        egui::vec2(cell_size, cell_size), 
                                                        egui::Sense::click_and_drag()
                                                    );

                                                    let bg_color = if response.hovered() {
                                                        match state {
                                                            LetterState::Neutral => Color32::from_rgb(60, 60, 60),
                                                            LetterState::Selected => Color32::from_rgb(52, 143, 255),
                                                            LetterState::Rejected => Color32::from_rgb(220, 70, 70),
                                                        }
                                                    } else {
                                                        match state {
                                                            LetterState::Neutral => Color32::from_rgb(44, 44, 44),
                                                            LetterState::Selected => Color32::from_rgb(24, 119, 242),
                                                            LetterState::Rejected => Color32::from_rgb(190, 45, 45),
                                                        }
                                                    };

                                                    ui.painter().rect_filled(rect, 6.0, bg_color);

                                                    ui.painter().text(
                                                        rect.center() - egui::vec2(0.0, 6.0),
                                                        egui::Align2::CENTER_CENTER,
                                                        cell.letter.to_string(),
                                                        egui::FontId::proportional(cell_size * 0.45),
                                                        Color32::WHITE
                                                    );

                                                    ui.painter().text(
                                                        rect.center_bottom() - egui::vec2(0.0, 4.0),
                                                        egui::Align2::CENTER_BOTTOM,
                                                        cell.adjacent_mines.to_string(),
                                                        egui::FontId::proportional(cell_size * 0.25),
                                                        Color32::from_rgb(170, 170, 170)
                                                    );

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

                                    let backspace_btn = ui.add_sized(
                                        egui::vec2(backspace_btn_width, letter_size),
                                        egui::Button::new("<"),
                                    );
                                    if backspace_btn.clicked() {
                                        self.current_guess.pop();
                                    }

                                    ui.add_space(letter_spacing);

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
                                    let frame_color = Color32::from_rgb(30, 120, 255);
                                    for letter in selected_letters {
                                        let button = egui::Button::new(
                                            egui::RichText::new(letter.to_string()).color(Color32::WHITE)
                                        )
                                        .fill(frame_color);

                                        let response = ui.add_sized(egui::vec2(letter_size, letter_size), button);

                                        if response.clicked() && self.current_guess.len() < target_len {
                                            self.current_guess.push(letter.to_ascii_uppercase());
                                        }
                                        ui.add_space(letter_spacing);
                                    }
                                },
                            );

                            ui.add_space(20.0);
                        });
                    });
            });

            if game_over {
                ui.painter().rect_filled(
                    ui.max_rect(),
                    0.0,
                    Color32::from_rgba_unmultiplied(0, 0, 0, 200),
                );

                egui::Window::new("Game Over Popup")
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .title_bar(false)
                    .resizable(false)
                    .collapsible(false)
                    .frame(egui::Frame::window(&ctx.style())
                        .fill(Color32::from_rgb(30, 30, 30))
                        .stroke(egui::Stroke::new(2.0, Color32::from_rgb(100, 100, 100)))
                        .inner_margin(30.0)
                    )
                    .show(ctx, |ui| {
                        ui.set_max_width(360.0);
                        
                        ui.vertical_centered(|ui| {
                            if won {
                                ui.heading(egui::RichText::new("You Win!").color(Color32::GREEN).size(28.0));
                                ui.add_space(15.0);
                            } else {
                                ui.heading(egui::RichText::new("Game Over").color(Color32::LIGHT_RED).size(28.0));
                                ui.add_space(15.0);
                                ui.label(egui::RichText::new(format!(
                                    "You ran out of attempts.\n\nHidden word:\n{}", 
                                    self.game_state.target_word
                                )).size(16.0));
                            }

                            ui.add_space(20.0);
                            ui.label(egui::RichText::new(format!("Your time: {}", time_string)).size(16.0));
                            ui.add_space(25.0);

                            let new_game_btn = ui.add_sized(
                                egui::vec2(180.0, 40.0),
                                egui::Button::new(egui::RichText::new("Play again").size(18.0))
                            );

                            if new_game_btn.clicked() {
                                let current_diff = self.game_state.difficulty;
                                self.game_state = GameState::new_game(current_diff);
                                self.current_guess.clear();
                                self.past_guesses.clear();
                                self.final_time = None;
                            }
                        });
                    });
            }
        });
    }
}