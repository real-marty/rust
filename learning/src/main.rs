fn main() {
    let sentence = String::from("penys quick brown fox jumps over the lazy dog");
    let first_word = get_first_word(sentence);
    println!("First word: {}", first_word);
}

fn get_first_word(s: String) -> String {
    let mut first_word = String::new();
    for char in s.chars() {
        first_word.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return first_word;
}
