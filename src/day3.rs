pub fn main() {
    println!("This is day 3!");
    println!("Enter a number:");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Please enter a valid number");
    let mut count = 0;
    for i in 1..=number {
        count += i;
    }
    println!("The count from 1 to {} is {}", number, count);

}
