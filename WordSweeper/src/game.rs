use rand::Rng;

pub enum CellState {
    Neutral,
    Selected,
    Rejected,
}

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