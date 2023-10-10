use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut vec: Vec<i32> = Vec::<i32>::new();
    let mut mp: HashMap<i32, i32> = HashMap::new();

    fill_vec(&mut vec);
    let avrg: i32 = average(&vec);
    println!("Vector: {:?}", vec);

    vec.sort();
    let mediana: i32 = find_mediana(&vec);
    let mode_num: i32 = mode(&vec, &mut mp);
    
    println!("Sorted vector is {:?}", vec);
    println!("Average num is: {}", avrg);
    println!("Mediana in sorted array is: {}", mediana);
    println!("Mode is: {}", mode_num);
}

fn mode(vec: &Vec<i32>, mp: &mut HashMap<i32, i32>) -> i32 {
    for elem in vec.iter() {
        *mp.entry(*elem).or_insert(0) += 1;
    }
    let mut result: (i32, i32) = (0, 0);
    for (key, value) in mp.iter() {
        if result.1 < *value {
            result.1 = *value;
            result.0 = *key;
        }
    }
    result.0
}

fn average(vec: &Vec<i32>) -> i32 {
    let sum = vec.iter().fold(0, |acc, &x| acc + x);
    sum / vec.len() as i32
}

fn find_mediana(vec: &Vec<i32>) -> i32 {
    let size = vec.len();
    if size & 1 == 1 {
        *vec.get(size / 2).unwrap()
    } else {
        let left: i32 = *vec.get(size / 2 - 1).unwrap();
        let right: i32 = *vec.get(size / 2).unwrap();
        (right + left) / 2
    }
}

fn fill_vec(vec: &mut Vec<i32>) {
    for _i in 0..10 {
        let el: i32 = rand::thread_rng().gen_range(-10..=10);
        vec.push(el);
    }
}
