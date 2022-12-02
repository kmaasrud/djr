use djr::parse::Parser;

fn main() {
    let input = "";
    let parser = Parser::new(input);

    for _event in parser {
        todo!("Do something with event");
    }
}
