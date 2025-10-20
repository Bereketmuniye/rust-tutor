pub fn main(){
    println!("Day 18 Count Vowels");
    let iterator = true;

    while iterator{
        println!("enter the any word as you can");

        let mut word=String::new();
        std::io::stdin().read_line(&mut word).expect("error parsing");
        let mut split = word.split("");
        let mut count =0;
        for i in split{
            if (i =="a"||i=="A"||i=="e"||i=="E"||i=="i"||i=="I"||i=="o"||i=="O"||i=="u"||i=="U"){
                count+=1;
            }
        }
        println!("counter {}",count);
    }

}