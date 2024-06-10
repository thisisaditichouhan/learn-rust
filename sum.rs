//i64 is the 64-bit signed integer type
use std::io; //This brings the io module from Rust's standard library into scope, allowing us to use input r output functionalities like reading from the standard input.
fn main() {
    println!("Hello, world!");
    let a = read_number("Enter the first number (a):");
    let b = read_number("Enter the second number (b):");

    addition(a,b);
  
}

fn addition(a:i64,b:i64){
  let addtn = a + b;
  println!("The addition is ={}",addtn);
}
fn read_number(prompt: &str)->i64{
  loop{
    let mut input = String::new();
    println!("{}",prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<i64>(){
      Ok(num)=>return num,
      Err(_)=>println!("Please enter a valid number!")
    }
  }
}
