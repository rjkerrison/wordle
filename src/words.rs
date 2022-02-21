use rand::seq::SliceRandom;
use std::fs; // 0.7.2

pub fn get_word() -> Option<String> {
    let vs = get_words();
    Some(vs.choose(&mut rand::thread_rng())?.to_owned())
}

fn get_words() -> Vec<String> {
    let file = fs::File::open("words.json").expect("file should open read only");
    let json: Vec<String> = serde_json::from_reader(file).expect("file should be proper JSON");
    json
}
