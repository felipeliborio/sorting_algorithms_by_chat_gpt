function partition<T>(arr: T[], left: number, right: number, pivotIndex: number): number {
  const pivotValue = arr[pivotIndex];
  [arr[pivotIndex], arr[right]] = [arr[right], arr[pivotIndex]];
  let storeIndex = left;
  for (let i = left; i < right; i++) {
    if (arr[i] < pivotValue) {
      [arr[storeIndex], arr[i]] = [arr[i], arr[storeIndex]];
      storeIndex++;
    }
  }
  [arr[right], arr[storeIndex]] = [arr[storeIndex], arr[right]];
  return storeIndex;
}

export function quickSort<T>(arr: T[]): T[] {
  if (arr.length <= 1) return arr;
  const ninther = Math.floor(arr.length / 9);
  const left = partition(
    arr,
    0,
    arr.length - 1,
    Math.floor(arr.length / 2)
  );
  const right = partition(
    arr,
    left + 1,
    arr.length - 1,
    ninther
  );
  partition(arr, left + 1, right - 1, Math.floor((left + right) / 2));
  quickSort(arr.slice(0, left));
  quickSort(arr.slice(left + 1, right));
  quickSort(arr.slice(right + 1));
  return arr;
}

import fs from "fs"

const input = fs.readFileSync(process.argv[2], "utf8")
const arr = input.split(" ").map((item) => parseInt(item))

let init = Date.now()
const sortedArr = quickSort(arr)
let end = Date.now()

let isCorrect = true
for(let i=0; i<sortedArr.length-1; ++i) {
  if(sortedArr[i] > sortedArr[i+1]) {
    isCorrect = false
    break
  }
}

process.stdout.write("typescript elapsed seconds "+(end-init)/1000+" | correct: "+isCorrect+"\n")

fs.writeFileSync(process.argv[2]+".pdqsort.out.typescript.txt", sortedArr.join(" "))
