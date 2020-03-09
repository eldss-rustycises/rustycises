use std::io;
use std::process;

use quiz_game::{QAPair, Quiz};

fn main() {
    let quiz = Quiz::from_csv("./problems.csv").unwrap_or_else(|e| {
        println!("Problem opening csv: {}", e);
        process::exit(1);
    });

    let mut correct = 0;
    for (index, QAPair { question, answer }) in quiz.question_list().iter().enumerate() {
        // Ask question
        println!("Question #{}: {}", index + 1, question);
        // Get answer
        let mut attempt = String::new();
        while let Err(_) = io::stdin().read_line(&mut attempt) {
            println!("Problem reading answer, try again.");
        }

        if answer.to_lowercase() == attempt.trim().to_lowercase() {
            correct += 1;
        }
    }

    println!("\nYou scored {}/{}", correct, quiz.question_list().len());
}
