use std::env;
use std::fs;
use std::time::Instant;
use std::io::{self, Write};

// Stable Quicksort
fn quicksort_stable<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
  if arr.len() <= 1 {
      return arr.to_vec();
  }
  let pivot_index = partition_stable(arr);
  let mut left = quicksort_stable(&arr[..pivot_index]);
  let right = quicksort_stable(&arr[pivot_index + 1..]);
  left.push(arr[pivot_index].clone());
  left.extend(right);
  left
}

fn partition_stable<T: Ord + Clone>(arr: &[T]) -> usize {
  let pivot_index = arr.len() - 1;
  let mut i = 0;
  for j in 0..pivot_index {
      if arr[j] <= arr[pivot_index] {
          i += 1;
      }
  }
  i
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
  input = quicksort_stable(&mut input);
  let elapsed = now.elapsed();
  
  let output: String = input.iter().map( |&entry| entry.to_string() + " ").collect();
  
  fs::write(file_path+".merge_sort.out.rust.txt", output.trim())
      .expect("Unable to write file");

  println!("rust elapsed seconds {}", elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
  io::stdout().flush().unwrap();
}
