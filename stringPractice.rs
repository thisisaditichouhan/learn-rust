use std::io;
fn main(){
  println!("Hello Coders!");

    let mut s1 = String::from("Hi");
    let mut s2 = String::from("Coders!");
    let s3 = format!("{} {}",s1,s2);

    let valueA = String::from("abc");
    let valueB = String::from("123");

    println!("a is {} and b is {}",valueA,ValueB);
    
    //String
    //Owned, growable, and mutable UTF-8 encoded string.
    //Heap-allocated, meaning it can dynamically grow in size as needed.
    //Best used when you need ownership of the string and may need to modify it.
    println!("This is the first string concatenation using `String`: {}",s3);
  
  loop{
    println!("Do you want to talk to me? (Yes/No)");

    let mut continue_input = String::new();
    io::stdin().read_line(&mut continue_input).expect("Failed to read input");

    if continue_input.trim().eq_ignore_ascii_case("no"){
      break;
    }
  }
}
