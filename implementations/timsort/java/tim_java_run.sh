if ! [[ -f ./Timsort.class ]] || test $2 = "-c"
then
    javac ./Timsort.java --release 19
fi

if [[ -f $1.timsort.out.java.txt ]]
then
    rm $1.timsort.out.java.txt
fi

time java Timsort $1 >> $1.timsort.time.txt
