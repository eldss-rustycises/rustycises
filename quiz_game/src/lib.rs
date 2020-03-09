use csv;
use serde::Deserialize;
use std::error::Error;

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
    pub fn from_csv(path: &str) -> Result<Quiz, Box<dyn Error>> {
        let mut questions = Vec::new();
        let mut rdr = csv::Reader::from_path(path)?;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn quiz_from_csv_success() {
        let filename = "quiz-from-csv-success.csv";
        // Setup test file
        // Need new scope for file processing so it will close
        {
            let mut file = File::create(filename).expect("Problem creating test file.");
            let csv_text = b"\
question,answer
1+1,2
2+2,4
";
            file.write_all(csv_text)
                .expect("Problem writing to test file");
        } // file closed here

        // ===== Test begins here =====

        let quiz = Quiz::from_csv(filename).expect("Problem creating Quiz");

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

        // Remove test file
        fs::remove_file(filename).expect("Problem removing tempfile");
    }
}
