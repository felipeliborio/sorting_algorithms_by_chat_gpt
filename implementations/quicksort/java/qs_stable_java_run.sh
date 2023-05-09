if ! [[ -f ./QuickSortStable.class ]] || test $2 = "-c"
then
    javac ./QuickSortStable.java --release 19
fi

if [[ -f $1.quicksort_stable.out.java.txt ]]
then
    rm $1.quicksort_stable.out.java.txt
fi

# time java QuickSortStable $1 >> $1.quicksort_stable.time.txt
\time -v -a -f "Program: %C\nTotal time: %E\nUser Mode (s) %U\nKernel Mode (s) %S\nCPU: %P" java QuickSortStable $1
