fn main() {
    let word = String::from("hello world!");
    let word_s = first_word(&word);
    println!("{word_s}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}