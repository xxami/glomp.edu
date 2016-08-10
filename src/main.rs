
mod kana_quiz;
use kana_quiz::question_engine::Asker;

fn main() {
    let answer = Asker::ask("type: ザワルド", "ザワルド");
    if answer.correct {
        println!("correct!");
    } 
}
