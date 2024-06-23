use std::io;
fn main() {
    println!("Enter the string whose number of charcaters you want to know");
    let mut user_input_string = String::new();
    io::stdin()
        .read_line(&mut user_input_string)
        .expect("Failed to read input");

    println!("Count all characters or only printable characters?\na. All Characters\nb.Printable Characters only");
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read choice");

    let trimmed_input = user_input_string.trim_end();

    match user_choice.to_lowercase().trim() {
        "a" => {
            // Code for choice "a. All Charcters"

            println!(
                "The string you entered has {} characters.",
                trimmed_input.len()
            );
        }
        "b" => {
            // Code for choice "b.Printable Characters only"
            let printable_count = trimmed_input.chars().filter(|c| !c.is_whitespace()).count();

            println!("The string you entered has {} characters.", printable_count);
        }
        _ => {
            println!("Wrong choice! You had to choose either a or b")
        }
    }
}
