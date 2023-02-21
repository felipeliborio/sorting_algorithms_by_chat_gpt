import java.util.Arrays;

public class PatternDefeatingQuickSort {
    public static void main(String[] args) {
        int[] arr = {5, 2, 9, 3, 5, 8, 1, 3};
        patternDefeatingQuickSort(arr, 0, arr.length - 1);
        System.out.println(Arrays.toString(arr));
    }
    
    public static void patternDefeatingQuickSort(int[] arr, int start, int end) {
        if (start < end) {
            int pivot = selectPivot(arr, start, end);
            int[] partition = partition(arr, start, end, pivot);
            patternDefeatingQuickSort(arr, start, partition[0] - 1);
            patternDefeatingQuickSort(arr, partition[1] + 1, end);
        }
    }
    
    public static int selectPivot(int[] arr, int start, int end) {
        int length = end - start + 1;
        int middle = start + length / 2;
        int eighth = length / 8;
        int m1 = medianOfThree(arr, start, start + eighth, start + 2 * eighth);
        int m2 = medianOfThree(arr, middle - eighth, middle, middle + eighth);
        int m3 = medianOfThree(arr, end - 2 * eighth, end - eighth, end);
        return medianOfThree(arr, m1, m2, m3);
    }
    
    public static int medianOfThree(int[] arr, int i, int j, int k) {
        if (arr[i] < arr[j]) {
            if (arr[j] < arr[k]) {
                return j;
            } else if (arr[i] < arr[k]) {
                return k;
            } else {
                return i;
            }
        } else {
            if (arr[i] < arr[k]) {
                return i;
            } else if (arr[j] < arr[k]) {
                return k;
            } else {
                return j;
            }
        }
    }
    
    public static int[] partition(int[] arr, int start, int end, int pivot) {
        int i = start;
        int j = start;
        int k = end;
        while (j <= k) {
            if (arr[j] < pivot) {
                swap(arr, i, j);
                i++;
                j++;
            } else if (arr[j] > pivot) {
                swap(arr, j, k);
                k--;
            } else {
                j++;
            }
        }
        return new int[]{i, k};
    }
    
    public static void swap(int[] arr, int i, int j) {
        int temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }
}
