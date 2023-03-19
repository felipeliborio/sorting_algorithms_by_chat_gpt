if ! [[ -f ./QuickSortStable.class ]] || test $2 = "-c"
then
    javac ./QuickSortStable.java --release 19
fi

if [[ -f $1.quicksort_stable.out.java.txt ]]
then
    rm $1.quicksort_stable.out.java.txt
fi

time java QuickSortStable $1 >> $1.quicksort_stable.time.txt
