fn main() {
    let num = 10;
    let fizzbuzz = if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else {
        num.to_string()
    };
    println!("{}", fizzbuzz);
}
