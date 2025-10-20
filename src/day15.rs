
pub fn main(){
    println!("Day 15 practice question");

    let iterator = true;
    while iterator{
        println!("Enter the number you want to sum");
        let mut number = String::new();
        std::io::stdin().read_line(&mut number ).expect("Fail to Accept the Number");

        if number.trim()=="Q"|| number.trim()=="q"{
            println!("Good Baye");
            break;
        }

        let mut split = number.split("");
        let mut sum=0;

        for i in split{
            if i == ""{
                continue;
            }
            match i.trim().parse::<i32>() {
                Ok(n) => sum += n,
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            }

        }
        println!("{}",sum);

       

    }
}