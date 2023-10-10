use std::io;

fn main() {
    let mut s: String = String::new();
    
    io::stdin().
        read_line(&mut s).
        expect("Falied to read string");

    println!("String: {}", s);

    s = to_code(&mut s);
    println!("Coded string: {}", s)
}

fn to_code(s: &mut String) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut new_str: String = String::new();

    for word in words {
        let ch = word.chars().nth(0).unwrap();
        if is_vowel(ch) == false {
            if word.len() > 0 {
                new_str.push_str(rem_first_char(word));
                new_str.push(ch);
                new_str.push_str("ay");
            }
        } else {
            new_str.push_str(word);
            new_str.push_str("hay");
        }
    }
    new_str
}

fn rem_first_char(val: &str) -> &str {
    let mut chars = val.chars();
    chars.next();
    chars.as_str()
}

fn is_vowel(c: char) -> bool {
    match c.to_lowercase().next() {
        Some('a') | Some('e') | Some ('u') | Some('i') => true,
        _ => false,
    }
}
