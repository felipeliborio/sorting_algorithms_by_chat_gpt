fn insertion_sort<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) {
  for i in (left + 1)..=right {
      let key_item = arr[i];
      let mut j = i;
      while j > left && arr[j - 1] > key_item {
          arr[j] = arr[j - 1];
          j -= 1;
      }
      arr[j] = key_item;
  }
}

fn merge<T: PartialOrd + Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
  let len_left = mid - left + 1;
  let len_right = right - mid;
  let mut left_arr = vec![arr[left]; len_left];
  let mut right_arr = vec![arr[mid + 1]; len_right];
  for i in 0..len_left {
      left_arr[i] = arr[left + i];
  }
  for i in 0..len_right {
      right_arr[i] = arr[mid + i + 1];
  }
  let mut i = 0;
  let mut j = 0;
  let mut k = left;
  while i < len_left && j < len_right {
      if left_arr[i] <= right_arr[j] {
          arr[k] = left_arr[i];
          i += 1;
      } else {
          arr[k] = right_arr[j];
          j += 1;
      }
      k += 1;
  }
  while i < len_left {
      arr[k] = left_arr[i];
      i += 1;
      k += 1;
  }
  while j < len_right {
      arr[k] = right_arr[j];
      j += 1;
      k += 1;
  }
}

fn timsort<T: PartialOrd + Copy>(arr: &mut [T]) {
  let min_run = 32;
  let n = arr.len();
  for i in (0..n).step_by(min_run) {
      insertion_sort(arr, i, (i + min_run - 1).min(n - 1));
  }
  let mut size = min_run;
  while size < n {
      for start in (0..n).step_by(size * 2) {
          let mid = start + size - 1;
          let end = (start + size * 2 - 1).min(n - 1);
          merge(arr, start, mid, end);
      }
      size *= 2;
  }
}

fn main() {
  let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
  timsort(&mut arr);
  println!("{:?}", arr);
}
