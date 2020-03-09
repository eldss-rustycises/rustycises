use std::process;

use quiz_game::{QAPair, Quiz};

fn main() {
    let quiz = match Quiz::from_csv("./problems.csv") {
        Ok(q) => q,
        Err(e) => {
            println!("Problem opening csv: {}", e);
            process::exit(1);
        }
    };

    for QAPair { question, answer } in quiz.question_list.iter() {
        println!("{}? {}", question, answer);
    }
}
