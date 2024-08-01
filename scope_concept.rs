fn main() {
    let outside_variable = 5;
    {
        let inside_variable = 10;
        println!("Inside variable is: {}",inside_variable);
    }//inside variable goes out of scope here

    //println!("Inside variable is: {}",inside_variable);

    println!("Outside variable is: {}",outside_variable);

}
