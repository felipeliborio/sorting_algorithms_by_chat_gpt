use std::cmp::Ordering;

fn pdqsort<T: Ord>(arr: &mut [T]) {
  if arr.len() <= 1 {
      return;
  }
  if arr.len() < 16 {
      // Insertion sort for small arrays
      for i in 1..arr.len() {
          for j in (1..=i).rev() {
              if arr[j - 1] > arr[j] {
                  arr.swap(j - 1, j);
              } else {
                  break;
              }
          }
      }
      return;
  }
  let pivot = median_of_three(arr);
  let (mut i, mut j, mut k, mut p, mut q) = (0, 0, arr.len() - 1, 0, arr.len());
  loop {
      while j < q {
          let cmp = arr[j].cmp(&pivot);
          if cmp > Ordering::Less {
              break;
          } else if cmp == Ordering::Less {
              if i != j {
                  arr.swap(i, j);
              }
              i += 1;
          }
          j += 1;
      }
      while j < q {
          let cmp = arr[q - 1].cmp(&pivot);
          if cmp < Ordering::Greater {
              break;
          } else if cmp == Ordering::Greater {
              q -= 1;
              if arr[q] < pivot {
                  if i != j {
                      arr.swap(i, j);
                  }
                  if p != q {
                      arr.swap(j, q);
                  }
                  i += 1;
              } else {
                  arr.swap(j, q);
              }
              j -= 1;
          }
          q -= 1;
      }
      if i == j && j == k {
          break;
      }
      if i == j {
          j += 1;
      }
      if j == k {
          k -= 1;
      }
      arr.swap(i, j);
      if p == i {
          p = j;
      }
      if q == j {
          q = i;
      }
      i += 1;
      j -= 1;
  }
  let mut left_len = i - p;
  let mut right_len = k - j;
  if left_len > 1 {
      pdqsort(&mut arr[p..i]);
  }
  if right_len > 1 {
      pdqsort(&mut arr[j + 1..=k]);
  }
}

fn median_of_three<T: Ord>(arr: &[T]) -> &T {
  let n = arr.len();
  let m = n / 2;
  if n > 40 {
      // Tukey's "Ninther"
      let s = n / 8;
      median(&[median(&[arr[0], arr[s], arr[2 * s]]), median(&[arr[m - s], arr[m], arr[m + s]]), median(&[arr[n - 2 * s], arr[n - s], arr[n - 1]])])
  } else {
      median(&[arr[0], arr[m], arr[n - 1]])
  }
}

fn median<T: Ord>(arr: &[T]) -> &T {
  let n = arr.len();
  &arr[n / 2]
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    pdqsort(&mut arr);
    println!("{:?}", arr);
}
