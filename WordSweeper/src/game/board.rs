use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Cell {
    pub letter: char,
    pub is_mine: bool,
    pub adjacent_mines: u8,
}

pub fn generate_filled_grid(target_word: &str, grid_size: usize, rng: &mut impl Rng) -> Vec<Vec<Cell>> {
    let mut grid = vec![
        vec![
            Cell {
                letter: ' ',
                is_mine: false,
                adjacent_mines: 0,
            };
            grid_size
        ];
        grid_size
    ];

    let word_chars: Vec<char> = target_word.chars().collect();
    let unique_word_chars: HashSet<char> = target_word.chars().collect();
    let word_len = word_chars.len();

    let mut available_safe: Vec<char> = ('A'..='Z')
        .filter(|ch| !unique_word_chars.contains(ch))
        .collect();

    let safe_pool_size = (2 * word_len).min(available_safe.len());
    available_safe.shuffle(rng);
    available_safe.truncate(safe_pool_size);
    let chosen_safe_pool = available_safe;

    let mut available_positions: Vec<(usize, usize)> = Vec::with_capacity(grid_size * grid_size);
    available_positions.extend((0..grid_size).flat_map(|y| (0..grid_size).map(move |x| (x, y))));
    available_positions.shuffle(rng);

    for (i, &ch) in word_chars.iter().enumerate() {
        let (x, y) = available_positions[i];
        grid[y][x].letter = ch;
        grid[y][x].is_mine = true;
    }

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

pub fn calculate_adjacent_mines(grid: &mut Vec<Vec<Cell>>, grid_size: usize) {
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