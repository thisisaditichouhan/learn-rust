use std::io;

fn main() {
    println!("Articles");

    println!("Enter the word");
    let mut user_input_word = String::new();
    io::stdin()
        .read_line(&mut user_input_word)
        .expect("Failed to read input");

    let user_input_word = user_input_word.trim();

    let char_vec: Vec<char> = user_input_word.chars().collect();
    let ch = char_vec[0];

    let silent_h_words = [
        "hour",
        "hours",
        "hourly",
        "heir",
        "heirs",
        "heiress",
        "heiresses",
        "heirloom",
        "heirlooms",
        "honest",
        "honestly",
        "honesty",
        "honor",
        "honors",
        "honored",
        "honoring",
        "honorific",
        "honorably",
        "honorable",
        "honorably",
        "homage",
        "homages",
    ];

    let words_starting_not_sounding_like_vowel = ["europe", "university", "european", "one"];

    if words_starting_not_sounding_like_vowel.contains(&user_input_word) {
        println!("A {}", user_input_word);
    } else if silent_h_words.contains(&user_input_word) {
        println!("An {}", user_input_word);
    } else if ch == 'a'
        || ch == 'e'
        || ch == 'i'
        || ch == 'o'
        || ch == 'u'
        || ch == 'A'
        || ch == 'E'
        || ch == 'I'
        || ch == 'O'
        || ch == 'U'
    {
        println!("An {}", user_input_word);
    } else {
        println!("A {}", user_input_word);
    }
}
