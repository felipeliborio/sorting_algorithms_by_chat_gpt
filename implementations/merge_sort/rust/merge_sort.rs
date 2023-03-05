use std::env;
use std::io::{self, Write};

fn merge_sort(arr: &mut [i32]) {
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
    let mut input: Vec<i32> = env::args()
    .skip(1)
    .map(|x| x.parse().expect("Not a number!"))
    .collect();

    merge_sort(&mut input);

    let output: String = input.iter().map( |&entry| entry.to_string() + " ").collect();
    print!("sorted {}", output);

    io::stdout().flush().unwrap();
}
