import java.io.File;
import java.io.IOException;
import java.util.Arrays;
import java.util.Scanner;

public class RadixSort {
    public static void radixSort(int[] input) {
        if (input == null || input.length == 0) {
            return;
        }

        int max = input[0];
        for (int i = 1; i < input.length; i++) {
            if (input[i] > max) {
                max = input[i];
            }
        }

        for (int exp = 1; max / exp > 0; exp *= 10) {
            countingSort(input, exp);
        }
    }

    private static void countingSort(int[] arr, int exp) {
        int[] output = new int[arr.length];
        int[] count = new int[10];

        for (int i = 0; i < arr.length; i++) {
            count[(arr[i] / exp) % 10]++;
        }

        for (int i = 1; i < 10; i++) {
            count[i] += count[i - 1];
        }

        for (int i = arr.length - 1; i >= 0; i--) {
            int index = (arr[i] / exp) % 10;
            output[count[index] - 1] = arr[i];
            count[index]--;
        }

        for (int i = 0; i < arr.length; i++) {
            arr[i] = output[i];
        }
    }

    public static void main(String[] args) throws IOException {
        String filePath = args[0];

        File file = new File(filePath);

        Scanner sc = new Scanner(file);
        var inputStr = sc.nextLine().split(" ");
        sc.close();
        
        var input = new int[inputStr.length];
        for (int i = 0; i < inputStr.length; ++i) {
            input[i] = Integer.parseInt(inputStr[i]);
        }
        
        var now = System.currentTimeMillis();
        radixSort(input);
        var elapsed = System.currentTimeMillis() - now;

        String output = Arrays.toString(input)
            .replace(",", "");
        
        output = output.substring(1, output.length() - 1);
        
        File outputFile = new File(filePath+".radix_sort.out.java.txt");
        outputFile.createNewFile();
        var outputWriter = new java.io.PrintWriter(outputFile);
        outputWriter.print(output);
        outputWriter.close();

        boolean isCorrect = true;
        for (int i = 0; i < input.length - 1; ++i) {
            if (input[i] > input[i + 1]) {
                isCorrect = false;
                break;
            }
        }

        System.out.println("java elapsed seconds "+elapsed/1000.0+" | correct: "+isCorrect);
    }
}
