use std::io;
fn main()
{
  let p: i64 = 5;
  let q: i64 = 10;
  diff(p,q);

  loop
    {
    let p = read_input("Enter the first number (p):");
    let q = read_input("Enter the second number (q):");

    subtraction(p,q);

    let mut continue_input = String::new();
    println!("Do you want to perform another calculation? (Yes/No)");
    io::stdin().read_line(&mut continue_input).expect("Failed to read input");

    if continue_input.trim().eq_ignore_ascii_case("No"){
          break;
    }

    }
}
  
fn diff(p:i64,q:i64){
  let diff = p-q;
  println!("Difference of p and q = {}",diff);
}

fn subtraction(p:i64,q:i64){
  let subtract = p-q;
  println!("The subtraction result is ={}",subtract);
}

fn read_input(prompt: &str)->i64{
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
