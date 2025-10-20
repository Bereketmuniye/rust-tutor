pub fn main(){
    println!("Day 19 Prime Number Checker");

    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("error");

    let num:i32 = number.trim().parse().expect("error");

    let mut counter = 0;
    for i in 2..=num {
        if num%i == 0{
            counter +=1;
        }

    }
    if counter <2{
        println!("Prime");
    }else{
        println!("Not Prime");
    }

}