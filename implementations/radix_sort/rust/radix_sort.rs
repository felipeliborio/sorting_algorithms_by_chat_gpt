use std::env;
use std::fs;
use std::time::Instant;
use std::io::{self, Write};

fn radix_sort(arr: &mut [i32]) {
  if arr.len() <= 1 {
      return;
  }

  let mut radix = 1;
  let mut max = arr[0];

  for i in 1..arr.len() {
      if arr[i] > max {
          max = arr[i];
      }
  }

  while max / radix > 0 {
      counting_sort(arr, radix);
      radix *= 10;
  }
}

fn counting_sort(arr: &mut [i32], radix: i32) {
  let mut output = vec![0; arr.len()];
  let mut count = vec![0; 10];

  for i in 0..arr.len() {
      count[((arr[i] / radix) % 10) as usize] += 1;
  }

  for i in 1..10 {
      count[i] += count[i - 1];
  }

  for i in (0..arr.len()).rev() {
      let index = ((arr[i] / radix) % 10) as usize;
      output[count[index] - 1] = arr[i];
      count[index] -= 1;
  }

  for i in 0..arr.len() {
      arr[i] = output[i];
  }
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(&file_path)
        .expect("Error reading the file");

    let mut input: Vec<i32> = contents
        .split(" ")
        .map(|x| x.parse().expect("Not a number!"))
        .collect();

    let now = Instant::now();
    radix_sort(&mut input);
    let elapsed = now.elapsed();
    
    let output: String = input.iter().map( |&entry| entry.to_string() + " ").collect();
    
    fs::write(file_path+".merge_sort.out.rust.txt", output.trim())
        .expect("Unable to write file");

    println!("rust elapsed seconds {}", elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
    io::stdout().flush().unwrap();
}
