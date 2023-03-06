function radixSort(arr: number[]) {
  if (!arr || arr.length <= 1) {
    return arr;
  }

  const max = Math.max(...arr);
  for (let exp = 1; max / exp > 0; exp *= 10) {
    countingSort(arr, exp);
  }

  return arr;
}

function countingSort(arr: number[], exp: number) {
  const n = arr.length;
  const output = new Array(n);
  const count = new Array(10).fill(0);

  for (let i = 0; i < n; i++) {
    count[Math.floor(arr[i] / exp) % 10]++;
  }

  for (let i = 1; i < 10; i++) {
    count[i] += count[i - 1];
  }

  for (let i = n - 1; i >= 0; i--) {
    const index = Math.floor(arr[i] / exp) % 10;
    output[count[index] - 1] = arr[i];
    count[index]--;
  }

  for (let i = 0; i < n; i++) {
    arr[i] = output[i];
  }
}

const arr = process.argv.slice(2).map((item) => parseInt(item))
radixSort(arr);
process.stdout.write("sorted "+arr.join(" "))
