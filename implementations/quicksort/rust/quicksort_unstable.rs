use std::env;
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
  let mut input: Vec<i32> = env::args()
  .skip(1)
  .map(|x| x.parse().expect("Not a number!"))
  .collect();

  quicksort_unstable(&mut input);

  let output: String = input.iter().map( |&entry| entry.to_string() + " ").collect();
  print!("sorted {}", output);

  io::stdout().flush().unwrap();
}
