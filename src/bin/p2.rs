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
// * the `id` and `name` fields are required, the `email` field is optional.
// * check the documentation on the `std::fs::file` struct for reading
//   and writing files.
// * use the `split` function from the standard library to extract
//   specific fields.
// * try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * create your program starting at level 1. once finished, advance
//   to the next level.
// * make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

pub mod p2 {
    use std::collections::HashMap;
    use std::fs::{File, OpenOptions};
    use std::i64;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use thiserror::Error;

    /// Contact of Record
    #[derive(Debug)]
    pub struct Record {
        pub id: i64,
        pub name: String,
        pub email: Option<String>,
    }

    /// Contains all saved records.
    #[derive(Debug)]
    pub struct Records {
        inner: HashMap<i64, Record>,
    }

    impl Records {
        pub fn new() -> Self {
            Self {
                inner: HashMap::new(),
            }
        }
        pub fn add(&mut self, record: Record) {
            self.inner.insert(record.id, record);
        }

        /// return records sort by key
        pub fn into_vec(mut self) -> Vec<Record> {
            let mut records: Vec<_> = self.inner.drain().map(|kv| kv.1).collect();
            records.sort_by_key(|rec| rec.id);
            records
        }

        pub fn next_id(&self) -> i64 {
            let mut ids: Vec<_> = self.inner.keys().collect();
            ids.sort();
            match ids.pop() {
                Some(id) => id + 1,
                None => 1,
            }
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
    pub fn load_records(file_name: PathBuf, verbose: bool) -> std::io::Result<Records> {
        let mut file = File::open(file_name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(parse_recoads(buffer, verbose))
    }

    pub fn save_records(file_name: PathBuf, records: Records) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_name)?;
        file.write(b"id,name,email\n")?;
        for record in records.into_vec().into_iter() {
            // Optional field email
            let email = match record.email {
                Some(email) => email,
                None => "".to_string(),
            };
            let line = format!("{},{},{}\n", record.id, record.name, email);
            file.write(line.as_bytes())?;
        }
        file.flush()?; // ⚡
        Ok(())
    }
}

use p2::{load_records, save_records, Record};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "project2: contact manager")]
struct Opt {
    #[structopt(short, parse(from_os_str), default_value = "p2_data.csv")]
    data_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool,
}
#[derive(StructOpt, Debug)]
enum Command {
    Add {
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
    List {},
}

fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        // L2: I want to add new contacts.
        Command::Add { name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            let next_id = recs.next_id();
            recs.add(Record {
                id: next_id,
                name,
                email,
            });
            save_records(opt.data_file, recs)?;
        }

        // L1: I want to view my saved contacts.
        Command::List { .. } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            for record in recs.into_vec() {
                println!("{:?}", record);
            }
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        println!("an error occured: {},", e);
    }
}
