extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let pattern = r"^(Add|add|Show|show)$";
    let re = Regex::new(pattern).unwrap();
    let mut action: String = String::new();

    loop {
        action.clear();
        println!("Enter actoion:\n 1)Add\n2)Show");
        io::stdin().
            read_line(&mut action).
            expect("Failed to read");
        action = action.trim().to_string();

        if re.is_match(&action) {
            match action.as_str() {
                "Add" | "add" => add(&mut map), 
                "Show" | "show" => show(&map),
                _ => continue,
            }
        } else {
            println!("Enter please: Add (add) or Show (show)");
            continue;
        }
    }
}

fn add(mp: &mut HashMap<String, Vec<String>>) {
    let mut line: String = String::new();
    let mut line2: String = String::new();

    println!("Input name:");
    io::stdin().
            read_line(&mut line).
            expect("Failed to read");

    println!("Enter a department:");
    io::stdin().
            read_line(&mut line2).
            expect("Failed to read");
    
    let entry = mp.entry(line2.clone()).or_insert(vec![]);
    entry.push(line);
}

fn show(mp: &HashMap<String, Vec<String>>) {
    println!("{:?}", mp);
    for (key, names) in mp.iter() {
        println!("Department: {}", key);
        for name in names {
            println!(" - {}", name.trim());
        }
    }
}
