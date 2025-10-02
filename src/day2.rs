pub fn main() {
    println!("Enter the first number:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");
    println!("Enter the second number:");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");
    let sum = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, sum);

}