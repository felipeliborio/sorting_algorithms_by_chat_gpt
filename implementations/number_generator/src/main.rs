use rand::Rng;
use std::io::{self, Write};
use std::env; 

fn main() {
    let input: Vec<i32> = env::args()
        .skip(1)
        .map(|x| x.parse().expect("Not a number!"))
        .collect();

    let integers = generate_integers(input[0]);

    let output: String = integers.iter().map( |&entry| entry.to_string() + " ").collect();
    print!("{}", output.trim());

    io::stdout().flush().unwrap();
}

fn generate_integers(count: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut integers: Vec<i32> = Vec::new();
    for _i in 0..count {
        integers.push(rng.gen_range(0..2000000000));
    }
    integers
}
