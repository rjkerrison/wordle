use dialoguer::Input;
use serde::Serialize;

fn get_user_input(message: &str) -> String {
    Input::new().with_prompt(message).interact_text().unwrap()
}

#[derive(Serialize)]
struct LetterFeedback {
    char: char,
    result: Correctness,
}

#[derive(Serialize)]
enum Correctness {
    Correct,
    Position,
    Incorrect,
}

type WordFeedback = Vec<LetterFeedback>;

trait AllCorrect {
    fn all_correct(self) -> bool;
}

impl AllCorrect for WordFeedback {
    fn all_correct(self) -> bool {
        self.iter().all(|l| match l.result {
            Correctness::Correct => true,
            _ => false,
        })
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

fn compare_guess_to_word(guess: String, word_chars: &[char]) -> WordFeedback {
    guess
        .chars()
        .enumerate()
        .map(|(i, c): (usize, char)| compare_letter_to_word(c, i, word_chars))
        .collect()
}

fn guess(word_chars: &[char]) -> WordFeedback {
    let guess = get_user_input("guess");
    compare_guess_to_word(guess, word_chars)
}

fn main() {
    let word = "Robin".to_owned();
    let word_chars: Vec<char> = word.chars().into_iter().collect();

    let mut count = 0;
    while count < 6 {
        let result = guess(&word_chars);
        println!("{}", serde_json::to_string_pretty(&result).unwrap());

        if result.all_correct() {
            println!("Well done!");
            return;
        }

        count += 1;
    }
    println!("Bad luck!");
}
