if ! [[ -f ./quicksort_stable.app ]] || test $2 = "-c"
then
    rustc ./quicksort_stable.rs -o ./quicksort_stable.app -C opt-level=2
fi

if [[ -f $1.quicksort_stable.out.rust.txt ]]
then
    rm $1.quicksort_stable.out.rust.txt
fi

time ./quicksort_stable.app $1 >> $1.quicksort_stable.time.txt
