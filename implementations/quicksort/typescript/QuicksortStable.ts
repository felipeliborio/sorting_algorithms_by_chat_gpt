// Stable Quicksort
function quicksortStable<T extends Comparable<T>>(arr: T[]): T[] {
  if (arr.length <= 1) {
    return arr;
  }
  const pivotIndex = partitionStable(arr);
  const left = arr.slice(0, pivotIndex);
  const right = arr.slice(pivotIndex + 1);
  return merge(quicksortStable(left), arr[pivotIndex], quicksortStable(right));
}

function partitionStable<T extends Comparable<T>>(arr: T[]): number {
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

function merge<T extends Comparable<T>>(left: T[], pivot: T, right: T[]): T[] {
  const result = [...left, pivot, ...right];
  quicksortStableHelper(result, 0, left.length - 1);
  quicksortStableHelper(result, left.length + 1, result.length - 1);
  return result;
}
