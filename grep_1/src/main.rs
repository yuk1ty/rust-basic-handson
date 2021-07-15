use std::fs::read_to_string;

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(path: String, pattern: String) {
    match read_to_string(path) {
        Ok(content) => grep(content, pattern),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    let pattern = std::env::args().nth(1);
    let path = std::env::args().nth(2);

    match (pattern, path) {
        (Some(pattern), Some(path)) => run(path, pattern),
        _ => println!("pattern or path is not specified!"),
    }
}
