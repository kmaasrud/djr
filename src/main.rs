mod argparse;

use djr::parse::Parser;
use std::path::PathBuf;

fn main() {
    if argparse::help() {
        println!(
            "djr v{}\n{}",
            env!("DJR_VERSION"),
            include_str!("./help.txt")
        );
        std::process::exit(1);
    }

    if argparse::version() {
        println!(
            "djr v{} (cli v{})",
            env!("DJR_VERSION"),
            env!("CARGO_PKG_VERSION")
        );
        std::process::exit(1);
    }

    let mut file = PathBuf::new();
    for field in argparse::args() {
        match field {
            argparse::Arg::File(f) => file.push(f),
            _ => {}
        }
    }

    let input = std::fs::read_to_string(file).unwrap();
    let parser = Parser::new(&input);

    for _event in parser {
        todo!("Do something with event");
    }
}
