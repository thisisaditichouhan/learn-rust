//i64 is the 64-bit signed integer type
use std::io; //This brings the io module from Rust's standard library into scope, allowing us to use input r output functionalities like reading from the standard input.
//fn main() {} defines the main function that is the entry point of a rust program.
fn main() {
    println!("Hello, world!"); //println!("") is a macro that prints the strings
    let a = read_number("Enter the first number (a):");
    let b = read_number("Enter the second number (b):");

    addition(a,b);
  
}

fn addition(a:i64,b:i64){
  let addtn = a + b;
  println!("The addition is ={}",addtn);
}
fn read_number(prompt: &str)->i64{ //->i64 indicates that the function returns an `i64`
  loop{
    let mut input = String::new(); //mut indicates that a variable is mutable and can be changed
    println!("{}",prompt);
     //expect("Failed to read line"): If read_line fails, the program will panic and display the message "Failed to read line".
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<i64>(){
      Ok(num)=>return num,
      Err(_)=>println!("Please enter a valid number!")
    }
  }
}
