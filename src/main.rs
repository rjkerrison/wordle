use dialoguer::Input;
use wordle::{compare_guess_to_word, get_word_as_chars, AllCorrect, WordFeedback};

fn get_user_input(message: &str) -> String {
    Input::new().with_prompt(message).interact_text().unwrap()
}

fn guess(word_chars: &[char]) -> WordFeedback {
    let guess = get_user_input("guess").to_uppercase();
    compare_guess_to_word(guess, word_chars)
}

fn main() {
    let word_chars = get_word_as_chars();

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
