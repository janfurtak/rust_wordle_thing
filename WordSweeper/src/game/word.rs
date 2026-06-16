use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::Rng;

use super::state::Difficulty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LetterFeedback {
    Correct,
    Misplaced,
    Absent,
}

pub fn draw_target_word(game_difficulty: &Difficulty, rng: &mut impl Rng) -> String {
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
            vec!["ERROR".to_string()]
        }
    };

    words_list.choose(rng).unwrap().to_uppercase()
}

pub fn guess_word(guess: &str, target_word: &str) -> Vec<LetterFeedback> {
    let guess = guess.to_uppercase();
    let target_word = target_word.to_uppercase();
    
    let mut remaining_target_chars: HashMap<char, i8> = HashMap::new();
    for c in target_word.chars() {
        *remaining_target_chars.entry(c).or_insert(0) += 1;
    }

    // Inicjalizujemy wektor feedbacku domyślnymi wartościami (np. Absent)
    let mut feedback = vec![LetterFeedback::Absent; target_word.len()];
    let guess_chars: Vec<char> = guess.chars().collect();
    let target_chars: Vec<char> = target_word.chars().collect();

    // Szukamy tylko poprawnych trafień
    for i in 0..guess_chars.len() {
        if guess_chars[i] == target_chars[i] {
            feedback[i] = LetterFeedback::Correct;
            if let Some(count) = remaining_target_chars.get_mut(&guess_chars[i]) {
                *count -= 1;
            }
        }
    }

    // Szukamy liter na złych miejscach oraz nieobecnych
    for i in 0..guess_chars.len() {
        // Pomijamy pozycje, które już dostały zielony kolor
        if feedback[i] == LetterFeedback::Correct {
            continue;
        }

        let g = guess_chars[i];
        if let Some(count) = remaining_target_chars.get_mut(&g) {
            if *count > 0 {
                feedback[i] = LetterFeedback::Misplaced;
                *count -= 1;
                continue;
            }
        }
        
        feedback[i] = LetterFeedback::Absent;
    }

    feedback
}