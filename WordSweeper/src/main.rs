mod game;

use game::GameState;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let file_path = Path::new("assets").join("words.txt");
    
    let content = match read_to_string(&file_path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("file read error {}: {}", file_path.display(), err);
            return;
        }
    };

    let words: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let game_state = GameState::new(&words);

    println!("slowo: {}", game_state.target_word);

    println!("[X,n] - litera jest mina (czescia slowa),  X,n  - zwykla litera");
    println!("n - liczba min dookola\n");

    for row in &game_state.grid {
        for cell in row {
            if cell.is_mine {
                print!("[{},{}] ", cell.letter, cell.adjacent_mines);
            } else {
                print!(" {}{}  ", cell.letter, cell.adjacent_mines);
            }
        }
        println!();
    }
}