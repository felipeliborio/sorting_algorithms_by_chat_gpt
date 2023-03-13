import java.io.File;
import java.util.Arrays;
import java.util.Scanner;

public class MergeSort {
    public static void merge(long[] arr, int start, int mid, int end) {
        long[] temp = new long[end - start + 1];
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

    public static void mergeSortHelper(long[] arr, int start, int end) {
        if (start < end) {
            int mid = start + (end - start) / 2;
            mergeSortHelper(arr, start, mid);
            mergeSortHelper(arr, mid + 1, end);
            merge(arr, start, mid, end);
        }
    }

    public static void mergeSort(long[] arr) {
        int n = arr.length;
        if (n <= 1) {
            return;
        }
        mergeSortHelper(arr, 0, n - 1);
    }

    public static void main(String[] args) throws Exception {
        String filePath = args[0];

        File file = new File(filePath);

        Scanner sc = new Scanner(file);
        var inputStr = sc.nextLine().split(" ");
        sc.close();
        
        var input = new long[inputStr.length];
        for (int i = 0; i < inputStr.length; ++i) {
            input[i] = Long.parseLong(inputStr[i]);
        }
        
        var now = System.currentTimeMillis();
        mergeSort(input);
        var elapsed = System.currentTimeMillis() - now;

        String output = Arrays.toString(input)
            .replace(",", "");
        
        output = output.substring(1, output.length() - 1);
        
        File outputFile = new File(filePath+".merge_sort.out.java.txt");
        outputFile.createNewFile();
        var outputWriter = new java.io.PrintWriter(outputFile);
        outputWriter.print(output);
        outputWriter.close();

        System.out.println("java elapsed seconds "+elapsed/1000.0);
    }
}
