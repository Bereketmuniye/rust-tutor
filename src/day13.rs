fn modify_value(value:&mut i32){
    *value +=1;
}
pub fn main(){
    println!("This is Day 13!");
    let mut p =10;
    modify_value(&mut p);
   
    println!("The Value Of Is: {}",p);

    
}