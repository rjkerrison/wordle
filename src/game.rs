use crate::{compare_guess_to_word, get_word_as_chars, AllCorrect, WordFeedback};

use serde::Serialize;

const ATTEMPTS_LIMIT: usize = 6;

#[derive(Clone, Serialize)]
pub struct Attempt {
    guess: String,
    feedback: WordFeedback,
}

pub enum GameStatus {
    Open,
    Lost,
    Won,
}

pub struct Game {
    word: String,
    pub attempts: Vec<Attempt>,
    pub status: GameStatus,
}

impl Game {
    pub fn new() -> Self {
        Game {
            word: "Robin".to_owned(),
            attempts: vec![],
            status: GameStatus::Open,
        }
    }

    fn make_attempt(&self, guess: String) -> Attempt {
        Attempt {
            guess: guess.clone(),
            feedback: compare_guess_to_word(guess, &get_word_as_chars(self.word.clone())),
        }
    }

    pub fn try_guess(&mut self, guess: String) -> Attempt {
        let guess = guess.to_uppercase();
        let attempt = self.make_attempt(guess);

        if attempt.feedback.all_correct() {
            self.status = GameStatus::Won;
        }
        self.attempts.push(attempt.clone());
        if self.attempts.len() >= ATTEMPTS_LIMIT {
            self.status = GameStatus::Lost;
        }
        attempt
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
