use dialoguer::Input;
use wordle::game::{Game, GameStatus};

fn get_user_input(message: &str) -> String {
    Input::new().with_prompt(message).interact_text().unwrap()
}

fn main() {
    let mut game = Game::new();

    while matches!(&game.status, GameStatus::Open) {
        let guess = get_user_input("guess");
        let attempt = game.try_guess(guess);

        println!("{}", serde_json::to_string_pretty(&attempt).unwrap());
    }

    match game.status {
        GameStatus::Open => panic!("Something went wrong"),
        GameStatus::Lost => println!("Bad luck!"),
        GameStatus::Won => println!("Well done!"),
    }
}
