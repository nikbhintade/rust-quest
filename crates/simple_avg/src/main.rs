use std::io;

fn avg(numbers: &[i32]) -> f32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result as f32 / numbers.len() as f32
}

fn main() {
    println!("Input list of numbers separated by commas");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Oops!");

    let numbers: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
    // There are no variadic arguments in Rust
    let result = avg(&numbers);
    println!("The avg is {}", result);
}
