use warp::Filter;
use wordle::game::{Game, GameStatus};

#[tokio::main]
async fn main() {
    let game = Game::new();

    let h = game.clone();
    let guess: _ = warp::path!("guess" / String).map(move |guess| {
        let attempt = h.clone().try_guess(guess);
        serde_json::to_string_pretty(&attempt).unwrap()
    });

    let status: _ = warp::path!("status").map(move || match game.status {
        GameStatus::Open => "Open",
        GameStatus::Lost => "Lost",
        GameStatus::Won => "Won",
    });

    let routes = warp::get().and(guess.or(status));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
