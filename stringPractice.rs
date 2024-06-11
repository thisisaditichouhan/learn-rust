use std::io;
fn main(){
  println!("Hello Coders!");
  loop{
    println!("Do you want to talk to me? (Yes/No)");

    let mut continue_input = String::new();
    io::stdin().read_line(&mut continue_input).expect("Failed to read input");

    if continue_input.trim().eq_ignore_ascii_case("no"){
      break;
    }
  }
}
