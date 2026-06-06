use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
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
    pub letter_states: HashMap<char, CellState>,
    pub start_time: Instant,
}

impl GameState {
    pub fn new(words_list: &[String]) -> Self {
        let mut rng = rand::thread_rng();
        
        let target_word = words_list.choose(&mut rng).unwrap().to_uppercase();
        let word_len = target_word.chars().count();
        let grid_size = word_len + 1;
        
        let mut grid = vec![
            vec![
                Cell { letter: ' ', is_mine: false, adjacent_mines: 0 }; 
                grid_size
            ]; 
            grid_size
        ];

        // wybranie liter obecnych w slowie
        let word_chars: Vec<char> = target_word.chars().collect();
        let unique_word_chars: HashSet<char> = target_word.chars().collect();
        
        // pozostale litery (nieobecne)
        let all_safe_chars: Vec<char> = ('A'..='Z')
            .filter(|ch| !unique_word_chars.contains(ch))
            .collect();
            
        // wybieram 2 * N liter nieobecnych w slowie ktorymi wypelnie reszte
        // siatki. w ten sposob wiecej min jest zlinkowanych ze soba
        // ta logike mozna jeszcze pozmieniac
        let safe_pool_size = (2 * word_len).min(all_safe_chars.len());
        let mut available_safe = all_safe_chars.clone();
        available_safe.shuffle(&mut rng);
        let chosen_safe_pool: Vec<char> = available_safe.into_iter().take(safe_pool_size).collect();

        let mut available_positions: Vec<(usize, usize)> = (0..grid_size)
            .flat_map(|y| (0..grid_size).map(move |x| (x, y)))
            .collect();
        
        available_positions.shuffle(&mut rng);

        // wybieram kratki dla liter ze słowa
        for (i, ch) in target_word.chars().enumerate() {
            let (x, y) = available_positions[i];
            grid[y][x].letter = ch;
            grid[y][x].is_mine = true;
        }
        
        // teraz wypelniam wszystkie pozostale kratki (1/3 szansy ze wybierze
        // litere ze slowa) - wychodzi proporcja 1:2 liter ze slowa:liter z poza
        // tez do zmiany jesli trzeba nwm
        for pos in available_positions.iter().skip(word_len) {
            let (x, y) = *pos;
            
            if rng.gen_bool(1.0 / 3.0) {
                let random_word_char = *word_chars.choose(&mut rng).unwrap();
                grid[y][x].letter = random_word_char;
                grid[y][x].is_mine = true;
            } else {
                let random_safe_char = *chosen_safe_pool.choose(&mut rng).unwrap();
                grid[y][x].letter = random_safe_char;
                grid[y][x].is_mine = false;
            }
        }

        // liczenie ile min wokol kazdej kratki
        for y in 0..grid_size {
            for x in 0..grid_size {
                let mut count = 0;
                
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        
                        if ny >= 0 && ny < grid_size as isize && nx >= 0 && nx < grid_size as isize {
                            if grid[ny as usize][nx as usize].is_mine {
                                count += 1;
                            }
                        }
                    }
                }
                grid[y][x].adjacent_mines = count;
            }
        }

        // kazda litera ma status tak jak na tej stronce
        let mut letter_states = HashMap::new();
        for ch in 'A'..='Z' {
            letter_states.insert(ch, CellState::Neutral);
        }

        Self {
            target_word,
            grid_size,
            grid,
            letter_states,
            start_time: Instant::now(),
        }
    }
}