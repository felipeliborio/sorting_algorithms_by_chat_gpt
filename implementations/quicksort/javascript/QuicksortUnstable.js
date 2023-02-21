// Unstable Quicksort
function quicksortUnstable(arr) {
  quicksortUnstableHelper(arr, 0, arr.length - 1);
}

function quicksortUnstableHelper(arr, left, right) {
  if (left >= right) {
    return;
  }
  const pivotIndex = partitionUnstable(arr, left, right);
  quicksortUnstableHelper(arr, left, pivotIndex - 1);
  quicksortUnstableHelper(arr, pivotIndex + 1, right);
}

function partitionUnstable(arr, left, right) {
  const pivotValue = arr[right];
  let i = left;
  for (let j = left; j < right; j++) {
    if (arr[j].compareTo(pivotValue) < 0) {
      [arr[i], arr[j]] = [arr[j], arr[i]];
      i++;
    }
  }
  [arr[i], arr[right]] = [arr[right], arr[i]];
  return i;
}
