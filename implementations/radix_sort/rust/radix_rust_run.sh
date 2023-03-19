if ! [[ -f ./radix_sort.app ]] || test $2 = "-c"
then
    rustc ./radix_sort.rs -o ./radix_sort.app -C opt-level=2
fi

if [[ -f $1.radix_sort.out.rust.txt ]]
then
    rm $1.radix_sort.out.rust.txt
fi

time ./radix_sort.app $1 >> $1.radix_sort.time.txt
