import java.io.File;
import java.io.IOException;
import java.util.Arrays;
import java.util.Scanner;
import java.util.Random;

public class QuickSortUnstable {
    public static void main(String[] args) throws IOException {
        // String filePath = args[0];
        // File file = new File(filePath);
        
        // Scanner sc = new Scanner(file);
        // var inputStr = sc.nextLine().split(" ");
        // sc.close();
        
        // var input = new Integer[inputStr.length];
        // for (int i = 0; i < inputStr.length; ++i) {
        //     input[i] = Integer.parseInt(inputStr[i]);
        // }
        
        var length = 5000000;
        Random generator = new Random();

        var input = new Integer[length];
        for (int i = 0; i < length; ++i) {
            input[i] = generator.nextInt();
        }
        
        // var now = System.currentTimeMillis();
        quicksortUnstable(input);
        // var elapsed = System.currentTimeMillis() - now;
        
        // String output = Arrays.toString(input)
        //     .replace(",", "");
        
        // output = output.substring(1, output.length() - 1);
        
        // File outputFile = new File(filePath+".quicksort_unstable.out.java.txt");
        // outputFile.createNewFile();
        // var outputWriter = new java.io.PrintWriter(outputFile);
        // outputWriter.print(output);
        // outputWriter.close();

        // var isCorrect = true;
        // for (int i = 0; i < input.length - 1; ++i) {
        //     if (input[i] > input[i + 1]) {
        //         isCorrect = false;
        //         break;
        //     }
        // }

        // System.out.println("java elapsed seconds "+elapsed/1000.0 + " | correct: " + isCorrect);
    }

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
}
