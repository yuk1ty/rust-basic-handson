fn fizzbuzz(num: i32) -> String {
    if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else {
        num.to_string()
    }
}

fn main() {
    let result = (0..100)
        .map(fizzbuzz)
        .fold(String::from(""), |acc, line| format!("{}\n{}", acc, line));
    println!("{}", result);
}
