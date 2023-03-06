use std::env;
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
  let mut input: Vec<i32> = env::args()
  .skip(1)
  .map(|x| x.parse().expect("Not a number!"))
  .collect();

  radix_sort(&mut input);

  let output: String = input.iter().map( |&entry| entry.to_string() + " ").collect();
  print!("sorted {}", output);

  io::stdout().flush().unwrap();
}
