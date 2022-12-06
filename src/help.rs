const HELP: &str = r#"
usage: djr [option] <file> ...

options:
    -h --help       show this text
    -v --version    show the version number
    -f --format     the output format {html|latex}
    -o --output     a file to write the output to. stdout if omitted
    --fmt           output pretty-formatted Djot"#;

pub(crate) fn help() {
    println!("{}", HELP);
}