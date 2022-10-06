use std::env;
use std::fs;
fn main() {
    let args: Vec<String>=env::args().collect();
    let content=fs::read_to_string(args[1].clone()).expect("Error about something");
}
