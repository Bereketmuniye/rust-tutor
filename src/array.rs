pub fn main(){
    println!("This is the array module.");

    //array with data type 
    let number : [i32;5]=[1,2,3,4,5];
    println!("Array of numbers is {:?}",number);

    //array without data type
    let numbers = [1,2,3,4,5];
    println!("array of numbers is {:?}",numbers);

    //array with defualt values 
    let numbers:[i32;5]=[3;5];
    println!("Array of numbers is {:?}",numbers);

    //accessing element of array
    let color=["red","orange","green","blue"];
    println!("1st Color: {}", color[0]);
    println!("2nd Color: {}", color[1]);
    println!("3rd Color: {}", color[2]);

    //mutable array
    let mut numbers:[i32;5]=[1,2,3,4,5];
    println!("original array {:?}",numbers);

    //change the value of the 3rd element 
    numbers[2]=0;
    println!("changed array is {:?}",numbers);

    //looping the array
    let colors=["red","green","blue"];
    //loop through index
    for index in 0..3{
        println!("Index : {} --value: {}",index,colors[index]);
    }


    //practice question on array 

    //1.sum of array 
    let num =[1,2,3,4,5];
    let mut sum = 0;
    for index in 0..5{
        sum +=num[index];
    }
    println!("The sum of array is {}",sum);

    //find the maximum from array
    let mut max =0;
    for index in 0..5{
        if(num[index]>max){
            max = num[index];
        }
    }
    println!("The maximum of array is {}",max);

    //reveres array 
    let mut temp = Vec::new();
    for i in (0..num.len()).rev(){
        temp.push(num[i]);
    }
    println!("The value of reveresed array is {:?}",temp);

    //count even number in the array
    for i in 0..5{
        if(num[i]%2==0){
            println!("The number {} is Even",num[i]);
        }
    }
    //array index access like first,middle and last
    let mut middle = 0;
    let size = num.len();
    if size%2==0{
        middle = (num[(size/2)-1] + num[size/2])/2;
    }
    else{
        middle = num[size/2];
    }
    println!("the first element of array is {}",num[0]);
    println!("the middle element is {}",middle);
    println!("the last element of array is {}",num[(size-1)]);

    //check for duplicates in array
    for i in 0..5{
        if(num[i]==num[i+1]){
            println!("the number is duplicate {}",num[i]);
        }
    }


}