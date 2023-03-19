if ! [[ -f ./quicksort_unstable.app ]] || test $2 = "-c"
then
    rustc ./quicksort_unstable.rs -o ./quicksort_unstable.app -C opt-level=2
fi

if [[ -f $1.quicksort_unstable.out.rust.txt ]]
then
    rm $1.quicksort_unstable.out.rust.txt
fi

time ./quicksort_unstable.app $1 >> $1.quicksort_unstable.time.txt
