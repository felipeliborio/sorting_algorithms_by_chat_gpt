public class RadixSort {
    public static void radixSort(int[] arr) {
        if (arr == null || arr.length == 0) {
            return;
        }

        int max = arr[0];
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] > max) {
                max = arr[i];
            }
        }

        for (int exp = 1; max / exp > 0; exp *= 10) {
            countingSort(arr, exp);
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

    public static void main(String[] args) {
        int[] input = new int[args.length];
        
        for (int i = 0; i < args.length; ++i) {
            input[i] = Integer.parseInt(args[i]);
        }

        radixSort(input);

        String output = "sorted "+input[0];
        for (int i = 1; i < input.length; ++i) {
            output += " "+input[i];
        }

        System.out.print(output);
    }
}
