use rand::Rng;

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
    let count = 5000000;
    let mut rng = rand::thread_rng();
    let mut integers: Vec<i32> = Vec::new();
    for _i in 0..count {
        integers.push(rng.gen_range(0..2000000000));
    }

    radix_sort(&mut integers);
}
