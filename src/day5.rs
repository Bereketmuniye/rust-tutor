pub fn main() {
    println!("This is day 5!");
    //convert integer to string
    let integer_value =42;
    let string_value = integer_value.to_string();
    println!("The string representation of the integer {} is '{}'", integer_value, string_value);

    //convert string to integer
    let string_value = "123";
    let integer_value: i32 = string_value.parse().expect("Not a valid integer");
    println!("The integer value of the string '{}' is {}", string_value, integer_value);

   
}