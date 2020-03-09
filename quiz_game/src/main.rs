use clap::{App, Arg};
use std::io;
use std::process;

use quiz_game::{QAPair, Quiz};

fn main() {
    // Define command line args
    let matches = App::new("Quiz Runner")
        .version("0.1.0")
        .author("Evan <evanldouglass@gmail.com")
        .about("Runs a quiz, parsed from a csv, on the command line.")
        .arg(
            Arg::with_name("csv_file")
                .short("c")
                .long("csv")
                .takes_value(true)
                .help(
                    "A csv file with lines in the form question,answer. \
                    Must have the header question,answer.",
                ),
        )
        .get_matches();

    let path_str = matches.value_of("csv_file").unwrap_or("./problems.csv");

    // Make quiz
    let quiz = Quiz::from_csv(path_str).unwrap_or_else(|e| {
        println!("Problem opening csv {}: {}", path_str, e);
        process::exit(1);
    });

    // Let user start when ready
    println!("Press Enter when ready");
    let _ = io::stdin().read_line(&mut String::new());

    // Start quiz
    let mut correct = 0;
    for (index, QAPair { question, answer }) in quiz.question_list().iter().enumerate() {
        // Ask question
        println!("Question #{}: {}", index + 1, question);
        // Get answer
        let mut attempt = String::new();
        while let Err(_) = io::stdin().read_line(&mut attempt) {
            println!("Problem reading answer, try again.");
        }

        // Check if correct
        if answer.trim().to_lowercase() == attempt.trim().to_lowercase() {
            correct += 1;
        }
    }

    // Display stats
    println!("\nYou scored {}/{}", correct, quiz.question_list().len());
}
