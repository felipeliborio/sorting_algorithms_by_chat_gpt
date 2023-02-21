import java.util.Arrays;

public class Timsort {

    private static final int MIN_MERGE = 32;
    private static final int MIN_GALLOP = 7;

    public static void sort(int[] arr) {
        int len = arr.length;
        if (len < 2) {
            return;
        }
        int minRun = minRunLength(len);
        for (int i = 0; i < len; i += minRun) {
            int end = Math.min(i + minRun, len);
            insertionSort(arr, i, end);
        }
        for (int size = minRun; size < len; size = 2 * size) {
            for (int left = 0; left < len; left += 2 * size) {
                int mid = left + size;
                int right = Math.min(left + 2 * size, len);
                merge(arr, left, mid, right);
            }
        }
    }

    private static void insertionSort(int[] arr, int left, int right) {
        for (int i = left + 1; i < right; i++) {
            int current = arr[i];
            int j = i - 1;
            while (j >= left && arr[j] > current) {
                arr[j + 1] = arr[j];
                j--;
            }
            arr[j + 1] = current;
        }
    }

    private static void merge(int[] arr, int left, int mid, int right) {
        int len1 = mid - left;
        int len2 = right - mid;
        int[] leftArr = Arrays.copyOfRange(arr, left, mid);
        int[] rightArr = Arrays.copyOfRange(arr, mid, right);
        int i = 0, j = 0, k = left;
        while (i < len1 && j < len2) {
            if (leftArr[i] <= rightArr[j]) {
                arr[k++] = leftArr[i++];
            } else {
                arr[k++] = rightArr[j++];
            }
        }
        while (i < len1) {
            arr[k++] = leftArr[i++];
        }
        while (j < len2) {
            arr[k++] = rightArr[j++];
        }
    }

    private static int minRunLength(int n) {
        int r = 0;
        while (n >= MIN_MERGE) {
            r |= (n & 1);
            n >>= 1;
        }
        return n + r;
    }

}
