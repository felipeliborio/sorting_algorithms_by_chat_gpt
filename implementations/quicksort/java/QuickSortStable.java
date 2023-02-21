// Stable Quicksort
public static <T extends Comparable<T>> T[] quicksortStable(T[] arr) {
    if (arr.length <= 1) {
        return arr;
    }
    int pivotIndex = partitionStable(arr);
    T[] left = Arrays.copyOfRange(arr, 0, pivotIndex);
    T[] right = Arrays.copyOfRange(arr, pivotIndex + 1, arr.length);
    return merge(quicksortStable(left), arr[pivotIndex], quicksortStable(right));
}

private static <T extends Comparable<T>> int partitionStable(T[] arr) {
    T pivotValue = arr[arr.length - 1];
    int i = 0;
    for (int j = 0; j < arr.length - 1; j++) {
        if (arr[j].compareTo(pivotValue) <= 0) {
            T temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            i++;
        }
    }
    T temp = arr[i];
    arr[i] = arr[arr.length - 1];
    arr[arr.length - 1] = temp;
    return i;
}

private static <T extends Comparable<T>> T[] merge(T[] left, T pivot, T[] right) {
    T[] result = Arrays.copyOf(left, left.length + 1 + right.length);
    result[left.length] = pivot;
    System.arraycopy(right, 0, result, left.length + 1, right.length);
    return result;
}
