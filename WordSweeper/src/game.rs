use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

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

#[derive(Clone)]
pub struct Cell {
    pub letter: char,
    pub is_mine: bool,
    pub adjacent_mines: u8,
}

pub struct GameState {
    pub target_word: String,
    pub grid_size: usize,
    pub grid: Vec<Vec<Cell>>,
    pub letter_states: HashMap<char, LetterState>,
    pub start_time: Instant,
}

impl GameState {
    pub fn new_game(game_difficulty: Difficulty) -> Self {
        let mut rng = rand::thread_rng();

        let target_word = Self::draw_target_word(&game_difficulty, &mut rng);
        
        let grid_size = match game_difficulty {
            Difficulty::Easy => 5,
            Difficulty::Medium => 7,
            Difficulty::Hard => 9,
        };

        // 1. Zbudowanie siatki z literami i minami
        let mut grid = Self::generate_filled_grid(&target_word, grid_size, &mut rng);

        // 2. Obliczenie min w sąsiedztwie
        Self::calculate_adjacent_mines(&mut grid, grid_size);

        // 3. Inicjalizacja statusów liter
        let mut letter_states = HashMap::new();
        for ch in 'A'..='Z' {
            letter_states.insert(ch, LetterState::Neutral);
        }

        Self {
            target_word,
            grid_size,
            grid,
            letter_states,
            start_time: Instant::now(),
        }
    }

    fn draw_target_word(game_difficulty: &Difficulty, rng: &mut rand::prelude::ThreadRng) -> String {
        let words_file: &str = match game_difficulty {
            Difficulty::Easy => "assets/words_easy.txt",
            Difficulty::Medium => "assets/words_medium.txt",
            Difficulty::Hard => "assets/words_hard.txt",
        };

        let words_list = match std::fs::read_to_string(words_file) {
            Ok(content) => content
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect::<Vec<String>>(),
            Err(err) => {
                eprintln!("file read error {}: {}", words_file, err);
                // Zwracamy jakiegoś fallbacka, żeby gra nie crashnęła
                vec!["ERROR".to_string()] 
            }
        };

        words_list.choose(rng).unwrap().to_uppercase()
    }

    /// ułożenie liter i min na planszy
    fn generate_filled_grid(target_word: &str, grid_size: usize, rng: &mut rand::prelude::ThreadRng) -> Vec<Vec<Cell>> {
        let mut grid = vec![
            vec![
                Cell {
                    letter: ' ',
                    is_mine: false,
                    adjacent_mines: 0
                };
                grid_size
            ];
            grid_size
        ];

        let word_chars: Vec<char> = target_word.chars().collect();
        let unique_word_chars: HashSet<char> = target_word.chars().collect();
        let word_len = word_chars.len();

        // Generujemy pulę bezpiecznych znaków
        let mut available_safe: Vec<char> = ('A'..='Z')
            .filter(|ch| !unique_word_chars.contains(ch))
            .collect();

        let safe_pool_size = (2 * word_len).min(available_safe.len());
        available_safe.shuffle(rng);
        available_safe.truncate(safe_pool_size);
        let chosen_safe_pool = available_safe;

        // Tasujemy pozycje na planszy
        let mut available_positions: Vec<(usize, usize)> = Vec::with_capacity(grid_size * grid_size);
        available_positions.extend((0..grid_size).flat_map(|y| (0..grid_size).map(move |x| (x, y))));
        available_positions.shuffle(rng);

        // Rozkładamy litery ze słowa (gwarantowane miny)
        for (i, &ch) in word_chars.iter().enumerate() {
            let (x, y) = available_positions[i];
            grid[y][x].letter = ch;
            grid[y][x].is_mine = true;
        }

        // wypełniamy resztę planszy
        for pos in available_positions.iter().skip(word_len) {
            let (x, y) = *pos;

            if rng.gen_bool(1.0 / 3.0) {
                grid[y][x].letter = *word_chars.choose(rng).unwrap();
                grid[y][x].is_mine = true;
            } else {
                grid[y][x].letter = *chosen_safe_pool.choose(rng).unwrap();
                grid[y][x].is_mine = false;
            }
        }

        grid
    }

    // zliczenie sąsiadujących min
    fn calculate_adjacent_mines(grid: &mut Vec<Vec<Cell>>, grid_size: usize) {
        let mut mine_positions = Vec::new();
        for y in 0..grid_size {
            for x in 0..grid_size {
                if grid[y][x].is_mine {
                    mine_positions.push((x, y));
                }
            }
        }

        for y in 0..grid_size {
            for x in 0..grid_size {
                let mut count = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let ny = y as isize + dy;
                        let nx = x as isize + dx;

                        // sprawdzamy granice planszy
                        if ny >= 0 && ny < grid_size as isize && nx >= 0 && nx < grid_size as isize {
                            if mine_positions.contains(&(nx as usize, ny as usize)) {
                                count += 1;
                            }
                        }
                    }
                }
                grid[y][x].adjacent_mines = count;
            }
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


pub enum LetterFeedback {
    Correct,
    Misplaced,
    Absent,
}

pub fn guess_word(guess: &str, target_word: &str) -> Vec<LetterFeedback> {
    let guess = guess.to_uppercase();
    
    let mut feedback = Vec::new();
    
    // Długość guess i target_word powinna być taka sama
    for (g, t) in guess.chars().zip(target_word.chars()) {
        if g == t {
            feedback.push(LetterFeedback::Correct);
        } else if target_word.contains(g) {
            feedback.push(LetterFeedback::Misplaced);
        } else {
            feedback.push(LetterFeedback::Absent);
        }
    }

    feedback
}