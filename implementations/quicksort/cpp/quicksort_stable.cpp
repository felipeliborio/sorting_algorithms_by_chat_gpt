// Stable Quicksort
template <typename T>
void quicksort_stable(T* arr, int left, int right) {
    if (left >= right) {
        return;
    }
    int pivot_index = partition_stable(arr, left, right);
    quicksort_stable(arr, left, pivot_index - 1);
    quicksort_stable(arr, pivot_index + 1, right);
}

template <typename T>
int partition_stable(T* arr, int left, int right) {
    T pivot_value = arr[right];
    int i = left;
    for (int j = left; j < right; j++) {
        if (arr[j] <= pivot_value) {
            std::swap(arr[i], arr[j]);
            i++;
        }
    }
    std::swap(arr[i], arr[right]);
    return i;
}
