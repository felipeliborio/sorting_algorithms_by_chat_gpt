use std::env;
use std::fs;
use std::time::Instant;
use std::io::{self, Write};

fn merge_sort(arr: &mut [i64]) {
  let len = arr.len();
  if len <= 1 {
      return;
  }
  let mid = len / 2;
  merge_sort(&mut arr[0..mid]);
  merge_sort(&mut arr[mid..len]);
  let mut temp = arr.to_vec();
  let mut i = 0;
  let mut j = mid;
  for k in 0..len {
      if i < mid && (j >= len || arr[i] <= arr[j]) {
          temp[k] = arr[i];
          i += 1;
      } else {
          temp[k] = arr[j];
          j += 1;
      }
  }
  arr.copy_from_slice(&temp);
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(&file_path)
        .expect("Error reading the file");

    let mut input: Vec<i64> = contents
        .split(" ")
        .map(|x| x.parse().expect("Not a number!"))
        .collect();

    let now = Instant::now();
    merge_sort(&mut input);
    let elapsed = now.elapsed();
    
    let output: String = input.iter().map( |&entry| entry.to_string() + " ").collect();
    
    fs::write(file_path+".merge_sort.out.rust.txt", output.trim())
        .expect("Unable to write file");

    println!("rust elapsed seconds {}", elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
    io::stdout().flush().unwrap();
}
