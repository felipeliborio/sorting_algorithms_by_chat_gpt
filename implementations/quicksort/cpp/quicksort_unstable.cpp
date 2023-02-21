// Unstable Quicksort
template <typename T>
void quicksort_unstable(T* arr, int left, int right) {
    if (left >= right) {
        return;
    }
    int pivot_index = partition_unstable(arr, left, right);
    quicksort_unstable(arr, left, pivot_index - 1);
    quicksort_unstable(arr, pivot_index + 1, right);
}

template <typename T>
int partition_unstable(T* arr, int left, int right) {
    T pivot_value = arr[right];
    int i = left;
    for (int j = left; j < right; j++) {
        if (arr[j] < pivot_value) {
            std::swap(arr[i], arr[j]);
            i++;
        }
    }
    std::swap(arr[i], arr[right]);
    return i;
}
