use lexer::regex::*;

fn write_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let regex = Regex::from(&args[1]);

    let strings = regex.to_strings();

    println!("{}", strings[0]);

    if let Err(error) = write_to_file("./img/nfa.dot", &strings[1]) {
        panic!("{}", error);
    }

    if let Err(error) = write_to_file("./img/dfa.dot", &strings[2]) {
        panic!("{}", error);
    }

    println!("{:?}", regex.matcher(&args[2], true));
}