function patternDefeatingQuicksort(arr) {
  if (arr.length <= 1) {
    return arr;
  }

  const pivot = arr[Math.floor(Math.random() * arr.length)];
  const [left, middle, right] = partition(arr, pivot);
  return [...patternDefeatingQuicksort(left), ...middle, ...patternDefeatingQuicksort(right)];
}

function partition(arr, pivot) {
  const lt = [];
  const eq = [];
  const gt = [];
  for (const x of arr) {
    if (x < pivot) {
      lt.push(x);
    } else if (x > pivot) {
      gt.push(x);
    } else {
      eq.push(x);
    }
  }
  return [lt, eq, gt];
}
