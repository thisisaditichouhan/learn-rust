
fn main() {
    println!("learning an array");
    let numberArray:[i32;10] = [1,2*7,3,4,5,6,7,8,9,10];
    arrayf(&numberArray);
    arrayDiffWays();
    
}

fn arrayDiffWays(){
    println!("printing array without loop using {:?}");
    let a = [4,5,6];
    println!("{:?}",a); 
    println!("printing one element specified number of times");
    let b = [2;8];
    println!("{:?}",b);
}

fn arrayf(numberArray:&[i32]){

    println!("Using .iter() in for loop");

    
    for n in numberArray.iter(){
        println!("{}",n);
    }
    println!("Using index");
    for i in 0..numberArray.len(){
        println!("{}",numberArray[i]);
    }
    
}

