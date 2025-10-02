pub fn main() {
    println!("This is Day 11!");
    
    //print factorial of the number 
    let mut fact = String::new();
    std::io::stdin().read_line(&mut fact).expect("Faild to find the number");
    let number = fact.trim().parse().expect("Faild to find parse");
    let mut factorial=1;
    for i in 1..=number{
        factorial *=i;
    }
    println!("The factorial of the number is : {}",factorial);
}