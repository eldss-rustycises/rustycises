use clap::{App, Arg, ArgGroup};
use std::io;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use quiz_game::{Msg, QAPair, Quiz};

const DEFAULT_SECS: u64 = 30;

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
        .arg(
            Arg::with_name("secs")
                .short("s")
                .long("secs")
                .takes_value(true)
                .help("The length of the quiz, in seconds."),
        )
        .arg(
            Arg::with_name("mins")
                .short("m")
                .long("mins")
                .takes_value(true)
                .help("The length of the quiz, in minutes."),
        )
        .group(
            ArgGroup::with_name("timer")
                .args(&["secs", "mins"])
                .required(false),
        )
        .get_matches();

    // CSV file path string
    let path_str = matches.value_of("csv_file").unwrap_or("./problems.csv");

    // Timer number
    let time: u64 = if matches.is_present("timer") {
        // Get string val
        matches
            .value_of("timer")
            .unwrap_or_else(|| {
                println!("Problem getting timer value.");
                process::exit(1);
            })
            // Parse to u32
            .parse()
            .unwrap_or_else(|e| {
                println!("Problem parsing time: {}", e);
                process::exit(1);
            })
    } else {
        DEFAULT_SECS
    };

    // Timer units
    let is_secs = if matches.is_present("mins") {
        false
    } else {
        true
    };

    // Make the timer
    let duration = if is_secs {
        Duration::from_secs(time)
    } else {
        Duration::from_secs(time * 60)
    };

    // Make quiz
    let quiz = Quiz::from_csv(path_str).unwrap_or_else(|e| {
        println!("Problem opening csv {}: {}", path_str, e);
        process::exit(1);
    });

    // Let user start when ready
    println!("Press Enter when ready");
    let _ = io::stdin().read_line(&mut String::new());

    // Data needed for program
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
    thread::spawn(move || {
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
