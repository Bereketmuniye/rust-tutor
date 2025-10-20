pub fn main(){
    println!("Day 16 Palindrome Checker");

    let iterator = true;

    while iterator{
        println!("Enter the string to check for palindrome");

        let mut palindrome=String::new();
        std::io::stdin().read_line(&mut palindrome).expect("error in your string");

       let reversed: String = palindrome.chars().rev().collect();

       if palindrome.trim() == reversed.trim(){
        println!("The string is {} palindrome",palindrome)
       }
       else {
        println!("The string is {} not palindrome",palindrome);
       }

    }
}