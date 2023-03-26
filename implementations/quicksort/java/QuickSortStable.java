import java.util.Arrays;
import java.io.File;
import java.io.IOException;
import java.util.Scanner;

public class QuickSortStable {
    public static void main(String[] args) throws IOException {
        String filePath = args[0];
        File file = new File(filePath);
        
        Scanner sc = new Scanner(file);
        var inputStr = sc.nextLine().split(" ");
        sc.close();
        
        var input = new Integer[inputStr.length];
        for (int i = 0; i < inputStr.length; ++i) {
            input[i] = Integer.parseInt(inputStr[i]);
        }
        
        var now = System.currentTimeMillis();
        input = quicksortStable(input);
        var elapsed = System.currentTimeMillis() - now;
        
        String output = Arrays.toString(input)
            .replace(",", "");
        
        output = output.substring(1, output.length() - 1);
        
        File outputFile = new File(filePath+".quicksort_stable.out.java.txt");
        outputFile.createNewFile();
        var outputWriter = new java.io.PrintWriter(outputFile);
        outputWriter.print(output);
        outputWriter.close();

        var isCorrect = true;
        for (int i = 0; i < input.length - 1; ++i) {
            if (input[i] > input[i + 1]) {
                isCorrect = false;
                break;
            }
        }

        System.out.println("java elapsed seconds "+elapsed/1000.0 + " | correct: " + isCorrect);
    }

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
}
