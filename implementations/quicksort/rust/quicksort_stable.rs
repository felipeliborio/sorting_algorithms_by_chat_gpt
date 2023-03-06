use std::env;
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
  let mut input: Vec<i32> = env::args()
  .skip(1)
  .map(|x| x.parse().expect("Not a number!"))
  .collect();

  let sorted = quicksort_stable(&mut input);

  let output: String = sorted.iter().map( |&entry| entry.to_string() + " ").collect();
  print!("sorted {}", output);

  io::stdout().flush().unwrap();
}
