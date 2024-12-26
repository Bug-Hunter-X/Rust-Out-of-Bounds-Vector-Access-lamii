fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Check if the index is within bounds before accessing.
    if let Some(number) = numbers.get(1) {
        println!("Value at index 1: {}", number);
    } else {
        println!("Index out of bounds");
    }

    // Alternatively, use the `get` method and handle the Option
    match numbers.get(10) {
        Some(number) => println!("Value at index 10: {}", number),
        None => println!("Index out of bounds"),
    }
} 