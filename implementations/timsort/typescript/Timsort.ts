function insertionSort<T>(array: T[], start: number, end: number, compare: (a: T, b: T) => number) {
  for (let i = start + 1; i <= end; i++) {
    const current = array[i];
    let j = i - 1;
    while (j >= start && compare(array[j], current) > 0) {
      array[j + 1] = array[j];
      j--;
    }
    array[j + 1] = current;
  }
}

function merge<T>(array: T[], start: number, mid: number, end: number, compare: (a: T, b: T) => number) {
  const leftLength = mid - start + 1;
  const rightLength = end - mid;
  const left = array.slice(start, mid + 1);
  const right = array.slice(mid + 1, end + 1);

  let leftIndex = 0;
  let rightIndex = 0;
  let arrayIndex = start;

  while (leftIndex < leftLength && rightIndex < rightLength) {
    if (compare(left[leftIndex], right[rightIndex]) <= 0) {
      array[arrayIndex] = left[leftIndex];
      leftIndex++;
    } else {
      array[arrayIndex] = right[rightIndex];
      rightIndex++;
    }
    arrayIndex++;
  }

  while (leftIndex < leftLength) {
    array[arrayIndex] = left[leftIndex];
    leftIndex++;
    arrayIndex++;
  }

  while (rightIndex < rightLength) {
    array[arrayIndex] = right[rightIndex];
    rightIndex++;
    arrayIndex++;
  }
}

function timsort<T>(array: T[], compare: (a: T, b: T) => number) {
  const minRun = 32;
  const n = array.length;

  for (let i = 0; i < n; i += minRun) {
    insertionSort(array, i, Math.min(i + minRun - 1, n - 1), compare);
  }

  for (let size = minRun; size < n; size = 2 * size) {
    for (let start = 0; start < n; start += 2 * size) {
      const mid = start + size - 1;
      const end = Math.min(start + 2 * size - 1, n - 1);
      if (mid < end) {
        merge(array, start, mid, end, compare);
      }
    }
  }
}

const arr: number[] = [];

for (let i = 0; i < 5000000; ++i) {
  arr.push(~~(2000000000 * Math.random()));
}

// import fs from "fs"

// const input = fs.readFileSync(process.argv[2], "utf8")
// const arr = input.split(" ").map((item) => parseInt(item))

// let init = Date.now()
timsort(arr, (a, b) => a-b)
// let end = Date.now()

// let isCorrect = true
// for (let i = 0; i < arr.length - 1; ++i) {
//   if (arr[i] > arr[i + 1]) {
//     isCorrect = false
//     break
//   }
// }
// process.stdout.write("typescript elapsed seconds "+(end-init)/1000+" | correct: "+isCorrect+"\n")

// fs.writeFileSync(process.argv[2]+".timsort.out.typescript.txt", arr.join(" "))
