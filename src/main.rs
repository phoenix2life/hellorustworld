use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");
    dependency_learn();
}


fn dependency_learn(){
    let stdout = stdout();
    let message = String::from("Hello Fellow Programmers, Welcome to Rust Programming!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

}