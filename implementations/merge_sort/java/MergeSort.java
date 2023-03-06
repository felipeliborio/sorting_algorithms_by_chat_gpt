public class MergeSort {

    public static void merge(int[] arr, int start, int mid, int end) {
        int[] temp = new int[end - start + 1];
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

    public static void mergeSortHelper(int[] arr, int start, int end) {
        if (start < end) {
            int mid = start + (end - start) / 2;
            mergeSortHelper(arr, start, mid);
            mergeSortHelper(arr, mid + 1, end);
            merge(arr, start, mid, end);
        }
    }

    public static void mergeSort(int[] arr) {
        int n = arr.length;
        if (n <= 1) {
            return;
        }
        mergeSortHelper(arr, 0, n - 1);
    }

    public static void main(String[] args) {
        int[] input = new int[args.length];
        
        for (int i = 0; i < args.length; ++i) {
            input[i] = Integer.parseInt(args[i]);
        }

        mergeSort(input);

        String output = "sorted "+input[0];
        for (int i = 1; i < input.length; ++i) {
            output += " "+input[i];
        }

        System.out.print(output);
    }
}
