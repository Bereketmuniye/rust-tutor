pub fn main (){
    println!("This is day 9!");
    
    // Print whether numbers from 0 to 20 are even or odd
    let mut count = 1;
    while count <=20{
        if count % 2 ==0 {
            println!("{} is even", count);
        } else {
            println!("{} is odd", count);
        }
        count +=1;
    }
}