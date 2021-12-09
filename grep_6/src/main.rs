use rayon::prelude::*;
use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

fn grep(content: &str, pattern: &str, path: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}: {}", path, line);
        }
    }
}

fn run(state: GrepArgs) {
    state
        .path
        .par_iter()
        .for_each(|file| match read_to_string(file) {
            Ok(content) => grep(&content, &state.pattern, file),
            Err(reason) => println!("{}", reason),
        });
}

fn main() {
    run(GrepArgs::from_args());
}
