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

fn compare_guess_to_word(guess: String, word: String) -> WordFeedback {
    let word_chars: Vec<char> = word.chars().into_iter().collect();
    guess
        .chars()
        .enumerate()
        .map(|(i, c): (usize, char)| compare_letter_to_word(c, i, &word_chars))
        .collect()
}

fn main() {
    let word = "Robin".to_owned();
    let guess = get_user_input("guess");
    let result = compare_guess_to_word(guess, word);

    println!("{}", serde_json::to_string_pretty(&result).unwrap())
}
