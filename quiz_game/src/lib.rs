use csv;
use rand;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::error::Error;
use std::io;

/// Quiz contains a list of questions and answers for a quiz.
pub struct Quiz {
    question_list: Vec<QAPair>,
}

/// QAPair stores a single record of a question/answer csv
#[derive(Debug, Deserialize, PartialEq)]
pub struct QAPair {
    pub question: String,
    pub answer: String,
}

/// Simple message enum for the quiz and timer to use.
pub enum Msg {
    Increment,
    Exit,
}

impl Quiz {
    /// Creates a new Quiz from a csv file. The file must have headers
    /// 'question' and 'answer', and only these headers.
    ///
    /// # Errors
    /// Function may fail while reading the csv or while deserializing data.
    pub fn from_reader(reader: impl io::Read) -> Result<Quiz, Box<dyn Error>> {
        let mut questions = Vec::new();
        let mut rdr = csv::Reader::from_reader(reader);

        for result in rdr.deserialize() {
            // transforms a record into a QAPair
            let record: QAPair = result?;
            questions.push(record);
        }

        let quiz = Quiz {
            question_list: questions,
        };

        Ok(quiz)
    }

    /// Returns a reference to the question list.
    pub fn question_list(&self) -> &Vec<QAPair> {
        &self.question_list
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.question_list[..].shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiz_from_reader_success() {
        let csv_text = "\
question,answer
1+1,2
2+2,4
";
        let quiz = Quiz::from_reader(csv_text.as_bytes()).expect("Problem creating Quiz");

        let qa_vec = vec![
            QAPair {
                question: String::from("1+1"),
                answer: String::from("2"),
            },
            QAPair {
                question: String::from("2+2"),
                answer: String::from("4"),
            },
        ];

        assert_eq!(qa_vec, quiz.question_list);
        assert_eq!(2, quiz.question_list.len());
    }
}
