// Stable Quicksort
function quicksortStable(arr) {
  if (arr.length <= 1) {
    return arr;
  }
  const pivotIndex = partitionStable(arr);
  const left = arr.slice(0, pivotIndex);
  const right = arr.slice(pivotIndex + 1);
  return merge(quicksortStable(left), arr[pivotIndex], quicksortStable(right));
}

function partitionStable(arr) {
  const pivotValue = arr[arr.length - 1];
  let i = 0;
  for (let j = 0; j < arr.length - 1; j++) {
    if (arr[j].compareTo(pivotValue) <= 0) {
      [arr[i], arr[j]] = [arr[j], arr[i]];
      i++;
    }
  }
  [arr[i], arr[arr.length - 1]] = [arr[arr.length - 1], arr[i]];
  return i;
}

function merge(left, pivot, right) {
  const result = [...left, pivot, ...right];
  quicksortStableHelper(result, 0, left.length - 1);
  quicksortStableHelper(result, left.length + 1, result.length - 1);
  return result;
}

function quicksortStableHelper(arr, left, right) {
  if (left >= right) {
    return;
  }
  const pivotIndex = partitionStableHelper(arr, left, right);
  quicksortStableHelper(arr, left, pivotIndex - 1);
  quicksortStableHelper(arr, pivotIndex + 1, right);
}

function partitionStableHelper(arr, left, right) {
  if (left === right) {
    return left;
  }
  const pivotIndex = Math.floor((left + right) / 2);
  const pivotValue = arr[pivotIndex];
  let i = left;
  let j = right;
  while (i < j) {
    while (arr[i].compareTo(pivotValue) < 0) {
      i++;
    }
    while (arr[j].compareTo(pivotValue) > 0) {
      j--;
    }
    if (i <= j) {
      [arr[i], arr[j]] = [arr[j], arr[i]];
      i++;
      j--;
    }
  }
  if (i === j && arr[i].compareTo(pivotValue) <= 0) {
    i++;
  }
  return i;
}
