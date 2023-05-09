use rand::Rng;
// Stable Quicksort
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

    let count = 5000000;
    let mut rng = rand::thread_rng();
    let mut integers: Vec<i32> = Vec::new();
    for _i in 0..count {
        integers.push(rng.gen_range(0..2000000000));
    }

    quicksort_unstable(&mut integers);
}
