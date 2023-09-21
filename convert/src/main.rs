use std::io;

fn main() {
    println!("Enter fagrenheit:");

    let fahr = read_fahr();

    println!("Fahr: {} -> Celsium: {}", fahr, convert_to_cel(fahr));
}

fn read_fahr() -> f64 {
    loop {
        let mut fahr: String = String::new();

        io::stdin()
            .read_line(&mut fahr)
            .expect("Failed to read!");

        let _fahr: f64 = match fahr.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn convert_to_cel(fahr: f64) -> f64 {
    (5.0/9.0) * (fahr - 32.0)
}
