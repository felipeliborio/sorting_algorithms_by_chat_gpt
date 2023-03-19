use std::env;
use std::fs;
use std::time::Instant;
use std::io::{self, Write};

// Unstable Quicksort
fn quicksort_unstable<T: Ord>(arr: &mut [T]) {
  if arr.len() <= 1 {
      return;
  }
  let pivot_index = partition_unstable(arr);
  quicksort_unstable(&mut arr[..pivot_index]);
  quicksort_unstable(&mut arr[pivot_index + 1..]);
}

fn partition_unstable<T: Ord>(arr: &mut [T]) -> usize {
  let pivot_index = arr.len() - 1;
  let mut i = 0;
  for j in 0..pivot_index {
      if arr[j] <= arr[pivot_index] {
          arr.swap(i, j);
          i += 1;
      }
  }
  arr.swap(i, pivot_index);
  i
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
  quicksort_unstable(&mut input);
  let elapsed = now.elapsed();
  
  let output: String = input.iter().map( |&entry| entry.to_string() + " ").collect();
  
  fs::write(file_path+".merge_sort.out.rust.txt", output.trim())
      .expect("Unable to write file");

  println!("rust elapsed seconds {}", elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
  io::stdout().flush().unwrap();
}
