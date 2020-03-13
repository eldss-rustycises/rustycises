use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

use quiz_game::{Msg, QAPair, Quiz};

// Holds command line arguments
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Quiz Runner",
    about = "Runs a quiz, parsed from a csv, on the command line."
)]
struct Opts {
    /// A CSV file path with question,answer fields.
    ///
    /// The file must have the headers question,answer
    /// in order for the program to parse it correctly.
    #[structopt(parse(from_os_str), short = "c", long = "csv")]
    csv_path: PathBuf,

    /// Randomly shuffle questions
    #[structopt(short, long)]
    random: bool,

    /// Set a time limit in seconds.
    #[structopt(short, long, default_value = "30")]
    secs: u64,

    /// Set a time limit in minutes.
    #[structopt(short, long, default_value = "0")]
    mins: u64,
}

fn main() {
    // Flags
    let opts = Opts::from_args();

    // Make the timer
    let duration = Duration::from_secs(opts.secs + (opts.mins * 60));

    // Open file
    let file = File::open(&opts.csv_path).unwrap_or_else(|e| {
        println!("Problem opening csv: {}", e);
        process::exit(1);
    });

    // Make quiz (mutable in case we need to shuffle)
    let mut quiz = Quiz::from_reader(file).unwrap_or_else(|e| {
        println!("Problem parsing csv: {}", e);
        process::exit(1);
    });

    // Let user start when ready
    println!("Press Enter when ready");
    let _ = io::stdin().read_line(&mut String::new());

    // Set up data/communication needed for program
    let mut correct = 0;
    let length = quiz.question_list().len();
    let (tx, rx) = mpsc::channel();

    // Start timer in new thread
    let timer_tx = tx.clone();
    thread::spawn(move || {
        thread::sleep(duration);
        timer_tx.send(Msg::Exit).expect("Problem sending message.");
    });

    // Start quiz in new thread
    let shuffle = opts.random;
    thread::spawn(move || {
        if shuffle {
            quiz.shuffle();
        }
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
                tx.send(Msg::Increment).expect("Problem sending message.");
            }
        }
        // Let main thread know the quiz is over
        tx.send(Msg::Exit).expect("Problem sending message.");
    });

    // Listen for messages from threads
    loop {
        match rx.recv() {
            Ok(Msg::Increment) => correct += 1,
            Ok(Msg::Exit) => break,
            Err(_) => {
                println!("Something went wrong. Exiting quiz.");
                break;
            }
        }
    }

    // Display stats
    println!("\nYou scored {}/{}", correct, length);
}
