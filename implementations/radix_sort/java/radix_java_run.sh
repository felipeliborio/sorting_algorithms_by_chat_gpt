if ! [[ -f ./RadixSort.class ]] || test $2 = "-c"
then
    javac ./RadixSort.java --release 19
fi

if [[ -f $1.radix_sort.out.java.txt ]]
then
    rm $1.radix_sort.out.java.txt
fi

time java RadixSort $1 >> $1.radix_sort.time.txt
