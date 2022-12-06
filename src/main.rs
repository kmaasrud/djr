mod argparse;
mod help;
mod version;

use std::path::PathBuf;

use djr::parse::Parser;

fn main() {
    if argparse::help() {
        version::version();
        help::help();
    }

    if argparse::version() {
        version::version();
    }

    let mut file = PathBuf::new();
    for field in argparse::args() {
        match field {
            argparse::Arg::File(f) => file.push(f),
            _ => {},
        }
    }

    let input = std::fs::read_to_string(file).unwrap();
    let parser = Parser::new(&input);

    for _event in parser {
        todo!("Do something with event");
    }
}
