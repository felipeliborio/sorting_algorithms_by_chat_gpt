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

import fs from "fs"

const input = fs.readFileSync(process.argv[2], "utf8")
const arr = input.split(" ").map((item) => parseInt(item))

let init = Date.now()
const sortedArr = mergeSort(arr)
let end = Date.now()
let isCorrect = true
for (let i = 0; i < sortedArr.length - 1; ++i) {
  if (sortedArr[i] > sortedArr[i + 1]) {
    isCorrect = false
    break
  }
}
process.stdout.write("typescript elapsed seconds "+(end-init)/1000+" | correct: "+isCorrect+"\n")

fs.writeFileSync(process.argv[2]+".merge_sort.out.typescript.txt", sortedArr.join(" "))
