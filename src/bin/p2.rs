// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

mod p2 {
    use std::collections::HashMap;
    use std::fs::File;
    use std::i64;
    use std::io::Read;
    use std::path::PathBuf;
    use thiserror::Error;

    /// Contact of Record
    #[derive(Debug)]
    struct Record {
        id: i64,
        name: String,
        email: Option<String>,
    }

    /// Contains all saved records.
    #[derive(Debug)]
    struct Records {
        inner: HashMap<i64, Record>,
    }

    impl Records {
        fn new() -> Self {
            Self {
                inner: HashMap::new(),
            }
        }
        fn add(&mut self, record: Record) {
            self.inner.insert(record.id, record);
        }
    }

    #[derive(Error, Debug)]
    enum ParseError {
        #[error("Id must be a number: {0}")]
        InvalidId(#[from] std::num::ParseIntError),
        #[error("empty record")]
        EmptyRecord,
        #[error("mission field: {0}")]
        MissingField(String),
    }

    fn parse_record(line: &str) -> Result<Record, ParseError> {
        let fields: Vec<&str> = line.split(',').collect();
        let id = match fields.get(0) {
            Some(id) => i64::from_str_radix(id, 10)?,
            None => return Err(ParseError::EmptyRecord),
        };
        let name = match fields.get(1).filter(|name| !name.is_empty()) {
            Some(name) => name.to_string(),
            None => return Err(ParseError::MissingField("name".to_owned())),
        };
        let email = fields
            .get(2)
            .map(|email| email.to_string())
            .filter(|email| !email.is_empty());

        Ok(Record { id, name, email })
    }
    fn parse_recoads(records: String, verbose: bool) -> Records {
        let mut recs = Records::new();
        for (num, record) in records.split('\n').enumerate() {
            if record != "" {
                match parse_record(record) {
                    Ok(rec) => recs.add(rec),
                    Err(e) => {
                        if verbose {
                            println!("error on line number {}: {} > \"{}\"\n", num + 1, e, record)
                        }
                    }
                }
            }
        }
        recs
    }
    fn load_records(file_name: PathBuf, verbose: bool) -> std::io::Result<Records> {
        let mut file = File::open(file_name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(parse_recoads(buffer, verbose))
    }
}
fn main() {}
