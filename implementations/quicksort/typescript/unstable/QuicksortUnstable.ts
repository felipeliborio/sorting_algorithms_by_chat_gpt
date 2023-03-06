// Unstable Quicksort
export function quicksortUnstable<T extends Comparable<T>>(arr: T[]): void {
  quicksortUnstableHelper(arr, 0, arr.length - 1);
}

function quicksortUnstableHelper<T extends Comparable<T>>(arr: T[], left: number, right: number): void {
  if (left >= right) {
    return;
  }
  const pivotIndex = partitionUnstable(arr, left, right);
  quicksortUnstableHelper(arr, left, pivotIndex - 1);
  quicksortUnstableHelper(arr, pivotIndex + 1, right);
}

function partitionUnstable<T extends Comparable<T>>(arr: T[], left: number, right: number): number {
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


const arr = process.argv.slice(2).map((item) => parseInt(item))
quicksortUnstable(arr)
process.stdout.write("sorted "+arr.join(" "))
