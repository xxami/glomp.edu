
mod kana_quiz;
use std::io;
use kana_quiz::question_engine::Asker;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let output = io::stdout();

    let mut asker = Asker::new(input, output);
    let answer = asker.ask("type: ザワルド", "ザワルド");
    if answer.correct {
        println!("correct!");
    } 
}
