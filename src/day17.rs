pub fn main(){
    println!("Day 17 Find Maximum Number in an Array");

    let arr = [1,9,5,7];

    let mut max=arr[0];


    for i in arr{
        if(i>max){
            max=i;

        }


    }

    println!("The max array value is {}",max);

}