pub mod game;
pub mod words;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct LetterFeedback {
    char: char,
    result: Correctness,
}

#[derive(Serialize, Clone)]
pub enum Correctness {
    Correct,
    Position,
    Incorrect,
}

pub type WordFeedback = Vec<LetterFeedback>;

pub trait AllCorrect {
    fn all_correct(&self) -> bool;
}

impl AllCorrect for WordFeedback {
    fn all_correct(&self) -> bool {
        self.iter()
            .all(|l| matches!(l.result, Correctness::Correct))
    }
}

fn compare_letter_to_word(c: char, i: usize, word: &[char]) -> LetterFeedback {
    if word[i] == c {
        return LetterFeedback {
            result: Correctness::Correct,
            char: c,
        };
    }
    if word.contains(&c) {
        return LetterFeedback {
            result: Correctness::Position,
            char: c,
        };
    }
    LetterFeedback {
        result: Correctness::Incorrect,
        char: c,
    }
}

pub fn compare_guess_to_word(guess: String, word_chars: &[char]) -> WordFeedback {
    guess
        .chars()
        .enumerate()
        .map(|(i, c): (usize, char)| compare_letter_to_word(c, i, word_chars))
        .collect()
}

pub fn get_word_as_chars(word: String) -> Vec<char> {
    let word_chars: Vec<char> = word.to_uppercase().chars().into_iter().collect();
    word_chars
}
