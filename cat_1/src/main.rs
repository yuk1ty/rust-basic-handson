use std::fs::read_to_string;

fn run_cat() {
    let path = "./src/main.rs";
    match read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    run_cat();
}
