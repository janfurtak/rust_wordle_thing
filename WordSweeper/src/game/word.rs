use rand::seq::SliceRandom;
use rand::Rng;

use super::state::Difficulty;

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
    let mut feedback = Vec::new();

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