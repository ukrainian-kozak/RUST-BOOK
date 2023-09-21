use rand::Rng;

fn main() {
    let n: i32 = rand::thread_rng().gen_range(1..=10);
    println!("{} -> Fib: {}", n, fibionachi(n));
}

fn fibionachi(n: i32) -> i32 {
    let mut one: i32 = 1;
    let mut two: i32 = 1;
    let mut tmp: i32 = 0;

    for _i in 2..n {
        tmp = one;
        one += two;
        two = tmp;
    }
    one
}
