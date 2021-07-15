use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: String,
}

fn grep(pattern: String, content: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(state: GrepArgs) {
    match read_to_string(state.path) {
        Ok(content) => grep(state.pattern, content),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    run(GrepArgs::from_args());
}
