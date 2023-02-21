#include <iostream>
#include <vector>

void merge(std::vector<int>& arr, int start, int mid, int end) {
    std::vector<int> temp(end - start + 1);
    int i = start, j = mid + 1, k = 0;

    while (i <= mid && j <= end) {
        if (arr[i] <= arr[j]) {
            temp[k++] = arr[i++];
        } else {
            temp[k++] = arr[j++];
        }
    }

    while (i <= mid) {
        temp[k++] = arr[i++];
    }

    while (j <= end) {
        temp[k++] = arr[j++];
    }

    for (i = start, k = 0; i <= end; ++i, ++k) {
        arr[i] = temp[k];
    }
}

void merge_sort_helper(std::vector<int>& arr, int start, int end) {
    if (start < end) {
        int mid = start + (end - start) / 2;
        merge_sort_helper(arr, start, mid);
        merge_sort_helper(arr, mid + 1, end);
        merge(arr, start, mid, end);
    }
}

void merge_sort(std::vector<int>& arr) {
    int n = arr.size();
    if (n <= 1) {
        return;
    }
    merge_sort_helper(arr, 0, n - 1);
}

int main() {
    std::vector<int> arr = {38, 27, 43, 3, 9, 82, 10};
    merge_sort(arr);
    for (int i = 0; i < arr.size(); ++i) {
        std::cout << arr[i] << " ";
    }
    std::cout << std::endl;
    return 0;
}
