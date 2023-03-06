export function mergeSort(arr: number[]): number[] {
  const n = arr.length;
  if (n <= 1) {
    return arr;
  }

  const mid = Math.floor(n / 2);
  const left = mergeSort(arr.slice(0, mid));
  const right = mergeSort(arr.slice(mid));

  return merge(left, right);
}

function merge(left: number[], right: number[]): number[] {
  const mergedArr: number[] = [];
  let i = 0;
  let j = 0;

  while (i < left.length && j < right.length) {
    if (left[i] < right[j]) {
      mergedArr.push(left[i]);
      i++;
    } else {
      mergedArr.push(right[j]);
      j++;
    }
  }

  while (i < left.length) {
    mergedArr.push(left[i]);
    i++;
  }

  while (j < right.length) {
    mergedArr.push(right[j]);
    j++;
  }

  return mergedArr;
}

const arr = process.argv.slice(2).map((item) => parseInt(item))
const sortedArr = mergeSort(arr)
process.stdout.write("sorted "+sortedArr.join(" "))
