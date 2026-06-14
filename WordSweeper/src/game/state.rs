use rand::thread_rng;
use std::collections::HashMap;
use std::time::Instant;

use super::board::{calculate_adjacent_mines, generate_filled_grid, Cell};
use super::word::draw_target_word;

#[derive(Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LetterState {
    Neutral,
    Selected,
    Rejected,
}

pub struct GameState {
    pub target_word: String,
    pub difficulty: Difficulty,
    pub grid_size: usize,
    pub grid: Vec<Vec<Cell>>,
    pub letter_states: HashMap<char, LetterState>,
    pub start_time: Instant,
}

impl GameState {
    pub fn new_game(game_difficulty: Difficulty) -> Self {
        let mut rng = thread_rng();

        let target_word = draw_target_word(&game_difficulty, &mut rng);

        let grid_size = match game_difficulty {
            Difficulty::Easy => 5,
            Difficulty::Medium => 7,
            Difficulty::Hard => 9,
        };

        let mut grid = generate_filled_grid(&target_word, grid_size, &mut rng);
        calculate_adjacent_mines(&mut grid, grid_size);

        let mut letter_states = HashMap::new();
        for ch in 'A'..='Z' {
            letter_states.insert(ch, LetterState::Neutral);
        }

        Self {
            target_word,
            difficulty: game_difficulty,
            grid_size,
            grid,
            letter_states,
            start_time: Instant::now(),
        }
    }

    pub fn handle_letter_click(&mut self, letter: char, is_right_click: bool) {
        let letter = letter.to_ascii_uppercase();

        if let Some(current_state) = self.letter_states.get_mut(&letter) {
            match (is_right_click, *current_state) {
                (false, LetterState::Selected) => *current_state = LetterState::Neutral,
                (false, _) => *current_state = LetterState::Selected,
                (true, LetterState::Rejected) => *current_state = LetterState::Neutral,
                (true, _) => *current_state = LetterState::Rejected,
            }
        }
    }

    pub fn get_selected_letters(&self) -> Vec<char> {
        self.letter_states
            .iter()
            .filter(|&(_, state)| *state == LetterState::Selected)
            .map(|(&ch, _)| ch)
            .collect()
    }
}