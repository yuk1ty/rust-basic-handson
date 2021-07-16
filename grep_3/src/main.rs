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

fn grep(state: &GrepArgs, content: String) {
    for line in content.lines() {
        if line.contains(state.pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(state: GrepArgs) {
    match read_to_string(&state.path) {
        Ok(content) => grep(&state, content),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    run(GrepArgs::from_args());
}
