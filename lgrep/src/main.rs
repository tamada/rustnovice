use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub trait Grepper {
    fn perform(&self, file: File) -> Vec<(usize, String)>;
}

pub struct CaseSensitiveGrepper {
    pattern: String,
}

impl CaseSensitiveGrepper {
    pub fn new(given_pattern: String) -> CaseSensitiveGrepper {
        CaseSensitiveGrepper {
            pattern: given_pattern.to_lowercase(),
        }
    }
}

impl Grepper for CaseSensitiveGrepper {
    fn perform(&self, file: File) -> Vec<(usize, String)> {
        let input = BufReader::new(file);
        let mut vec = Vec::new();
        for (index, line) in input.lines().enumerate() {
            let unwrapped_line = line.unwrap();
            let lower_line = &unwrapped_line.to_lowercase();
            if lower_line.contains(&self.pattern) {
                vec.push((index, unwrapped_line));
            }
        }
        vec
    }
}

pub struct DefaultGrepper {
    pattern: String,
}

impl DefaultGrepper {
    pub fn new(given_pattern: String) -> DefaultGrepper {
        DefaultGrepper {
            pattern: given_pattern,
        }
    }
}

impl Grepper for DefaultGrepper {
    fn perform(&self, file: File) -> Vec<(usize, String)> {
        let input = BufReader::new(file);
        let mut vec = Vec::new();
        for (index, line) in input.lines().enumerate() {
            let unwrapped_line = line.unwrap();
            if unwrapped_line.contains(&self.pattern) {
                vec.push((index, unwrapped_line));
            }
        }
        vec
    }
}

fn build(matches: ArgMatches) -> Box<dyn Grepper> {
    let pattern = matches.value_of("PATTERN").unwrap().to_string();
    if matches.is_present("ignore-case") {
        return Box::new(CaseSensitiveGrepper::new(pattern));
    }
    Box::new(DefaultGrepper::new(pattern))
}

fn perform_each(grepper: &Box<dyn Grepper>, file_name: &String) -> Vec<(usize, String)> {
    let file_path = Path::new(&file_name);
    let display = file_path.display();
    let file = match File::open(&file_path) {
        Err(why) => panic!("{}: {}", display, why.to_string()),
        Ok(f) => f,
    };
    grepper.perform(file)
}

fn print_result(file_name: String, results: Vec<(usize, String)>) {
    for result in results {
        println!("{} ({}): {}", file_name, result.0, result.1)
    }
}

// https://eh-career.com/engineerhub/entry/2017/07/19/110000#Rust%E3%81%A7%E5%AE%9F%E8%B7%B5%E7%9A%84%E3%81%AA%E5%AE%9F%E8%A3%85-%E3%81%9D%E3%81%AE1-rsgrep
fn perform(matches: ArgMatches) {
    let files = matches
        .values_of("FILEs")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let grepper = build(matches);
    for file in &files {
        let file_name = file.to_string();
        let vec = perform_each(&grepper, &file_name);
        print_result(file_name, vec);
    }
}

fn main() {
    let matches = App::new("lgrep")
        .version(crate_version!())
        .author(crate_authors!())
        .about("finds and prints the line which matches the given pattern.")
        .arg(
            Arg::with_name("fixed-string")
                .help("search PATTERN as a fixed string.")
                .long("fixed")
                .short("f"),
        )
        .arg(Arg::from_usage(
            "-i --ignore-case 'case sensitive matching'",
        ))
        .arg(
            Arg::with_name("PATTERN")
                .help("specifies the pattern for matching.")
                .required(true)
                .multiple(false)
                .index(1),
        )
        .arg(
            Arg::with_name("FILEs")
                .help("")
                .required(false)
                .multiple(true)
                .index(2),
        )
        .get_matches();
    perform(matches)
}
