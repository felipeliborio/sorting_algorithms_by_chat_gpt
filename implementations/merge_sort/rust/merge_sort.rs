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
  let mut arr = [38, 27, 43, 3, 9, 82, 10];
  merge_sort(&mut arr);
  println!("{:?}", arr);
}
