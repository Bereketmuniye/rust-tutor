pub fn main() {
    println!("This is day 6!");
    println!("Enter a first number:");
    let mut first_number = String::new();
    std::io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Please enter a valid number");
    println!("Enter a second number:");
    let mut second_number = String::new();
    std::io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Please enter a valid number");
    
    // Perform arithmetic operations
    let sum = first_number + second_number;
    let difference = first_number - second_number;
    let product = first_number * second_number;
    let quotient = if second_number != 0.0 {
        first_number / second_number
    } else {
        f64::NAN // Handle division by zero
    };
    println!("The sum of {} and {} is {}", first_number, second_number, sum);
    println!("The difference of {} and {} is {}", first_number, second_number, difference);
    println!("The product of {} and {} is {}", first_number, second_number, product);
    println!("The quotient of {} and {} is {}", first_number, second_number, quotient);
}