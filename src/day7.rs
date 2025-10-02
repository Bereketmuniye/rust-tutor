pub fn main() {
    println!("This is day 7!");

    //print even or odd
    println!("Enter a number:");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Please enter a valid number");
    if number % 2 == 0 {
        println!("{} is even", number);
    }if number == 1 {
        println!("{} is niether even nor odd", number);
    }
    else {
        println!("{} is odd", number);
    }
}