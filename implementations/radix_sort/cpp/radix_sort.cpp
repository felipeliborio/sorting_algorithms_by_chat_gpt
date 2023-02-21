#include <iostream>
#include <vector>

void counting_sort(std::vector<int>& arr, int radix) {
    std::vector<int> output(arr.size());
    std::vector<int> count(10);

    for (int i = 0; i < arr.size(); i++) {
        count[(arr[i] / radix) % 10]++;
    }

    for (int i = 1; i < 10; i++) {
        count[i] += count[i - 1];
    }

    for (int i = arr.size() - 1; i >= 0; i--) {
        int index = (arr[i] / radix) % 10;
        output[count[index] - 1] = arr[i];
        count[index]--;
    }

    for (int i = 0; i < arr.size(); i++) {
        arr[i] = output[i];
    }
}

void radix_sort(std::vector<int>& arr) {
    if (arr.size() <= 1) {
        return;
    }

    int max = arr[0];
    for (int i = 1; i < arr.size(); i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }

    int radix = 1;
    while (max / radix > 0) {
        counting_sort(arr, radix);
        radix *= 10;
    }
}

int main() {
    std::vector<int> arr = {170, 45, 75, 90, 802, 24, 2, 66};
    radix_sort(arr);

    for (int i = 0; i < arr.size(); i++) {
        std::cout << arr[i] << " ";
    }

    return 0;
}
