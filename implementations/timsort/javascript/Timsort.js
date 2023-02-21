function timsort(arr) {
  const MIN_MERGE = 32;

  function minRunLength(n) {
    let r = 0;
    while (n >= MIN_MERGE) {
      r |= n & 1;
      n >>= 1;
    }
    return n + r;
  }

  function insertionSort(arr, left, right) {
    for (let i = left + 1; i <= right; i++) {
      const temp = arr[i];
      let j = i - 1;
      while (j >= left && arr[j] > temp) {
        arr[j + 1] = arr[j];
        j--;
      }
      arr[j + 1] = temp;
    }
  }

  function merge(arr, l, m, r) {
    const len1 = m - l + 1, len2 = r - m;
    const left = [], right = [];

    for (let i = 0; i < len1; i++) {
      left[i] = arr[l + i];
    }
    for (let i = 0; i < len2; i++) {
      right[i] = arr[m + 1 + i];
    }

    let i = 0, j = 0, k = l;

    while (i < len1 && j < len2) {
      if (left[i] <= right[j]) {
        arr[k++] = left[i++];
      } else {
        arr[k++] = right[j++];
      }
    }

    while (i < len1) {
      arr[k++] = left[i++];
    }

    while (j < len2) {
      arr[k++] = right[j++];
    }
  }

  const n = arr.length;
  if (n < 2) {
    return;
  }

  const minRun = minRunLength(n);

  for (let i = 0; i < n; i += minRun) {
    insertionSort(arr, i, Math.min(i + minRun - 1, n - 1));
  }

  for (let size = minRun; size < n; size = 2 * size) {
    for (let left = 0; left < n; left += 2 * size) {
      const mid = left + size - 1;
      const right = Math.min(left + 2 * size - 1, n - 1);
      merge(arr, left, mid, right);
    }
  }
}
