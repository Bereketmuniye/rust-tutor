pub fn main() {
    println!("This is day 4!");
    println!("Enter a number:");
    let mut counter =0;
    for i in 1..=10 {
        counter += i;
        println!("Counter after adding {}: {}", i, counter);
    }

}