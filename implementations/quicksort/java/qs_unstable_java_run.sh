if ! [[ -f ./QuickSortUnstable.class ]] || test $2 = "-c"
then
    javac ./QuickSortUnstable.java --release 19
fi

if [[ -f $1.quicksort_unstable.out.java.txt ]]
then
    rm $1.quicksort_unstable.out.java.txt
fi

# time java QuickSortUnstable $1 >> $1.quicksort_unstable.time.txt
\time -v -a -f "Program: %C\nTotal time: %E\nUser Mode (s) %U\nKernel Mode (s) %S\nCPU: %P" java QuickSortUnstable $1
