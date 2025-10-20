
pub fn main(){
         println!("Day 14 work for fizzBuzz");
         
     let boolean = true;
      while boolean {
        let mut num = String::new();
         println!("Enter the number");    
         println!("press Q for quite the loop");
      
         std::io::stdin().read_line(&mut num).expect("invalide input ");

          if num.trim() == "Q" || num.trim()=="q"{
            println!("Good Baye");
            break
          }
          let number: i32 = num.trim().parse().expect("cannot be converted to numbe");

           if(number % 5 ==0 && number % 3 ==0){
            println!("The Number {} is fizzBuzz",number);
            }
            else if(number % 3==0){
                println!("The number is {} Fizz",number)
            }
            else if(number % 5 ==0){
                println!("The Number {} is Buzz",number);
            }
            else{
                println!("The number {} is niether ",number);
            }  
        

      }


}