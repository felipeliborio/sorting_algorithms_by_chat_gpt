if ! [[ -f ./merge_sort.app ]] || test $2 = "-c"
then
    javac ./MergeSort.java --release 19
fi

if [[ -f $1.merge_sort.out.java.txt ]]
then
    rm $1.merge_sort.out.java.txt
fi

time java MergeSort $1 >> $1.merge_sort.time.txt
