use rand::Rng;
use std::io::{self, Write};
use std::env; 

fn main() {
    let input: Vec<u32> = env::args()
        .skip(1)
        .map(|x| x.parse().expect("Not a number!"))
        .collect();

    let integers = generate_integers(input[0]);

    let output: String = integers.iter().map( |&entry| entry.to_string() + " ").collect();
    print!("{}", output);

    io::stdout().flush().unwrap();
}

fn generate_integers(count: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut integers = Vec::new();
    for _i in 0..count {
        integers.push(rng.gen());
    }
    integers
}
