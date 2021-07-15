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
    for num in 0..100 {
        println!("{}", fizzbuzz(num));
    }
}
