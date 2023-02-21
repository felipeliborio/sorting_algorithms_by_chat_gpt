// Unstable Quicksort
public static <T extends Comparable<T>> void quicksortUnstable(T[] arr) {
    quicksortUnstable(arr, 0, arr.length - 1);
}

private static <T extends Comparable<T>> void quicksortUnstable(T[] arr, int left, int right) {
    if (left >= right) {
        return;
    }
    int pivotIndex = partitionUnstable(arr, left, right);
    quicksortUnstable(arr, left, pivotIndex - 1);
    quicksortUnstable(arr, pivotIndex + 1, right);
}

private static <T extends Comparable<T>> int partitionUnstable(T[] arr, int left, int right) {
    T pivotValue = arr[right];
    int i = left;
    for (int j = left; j < right; j++) {
        if (arr[j].compareTo(pivotValue) < 0) {
            T temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            i++;
        }
    }
    T temp = arr[i];
    arr[i] = arr[right];
    arr[right] = temp;
    return i;
}
