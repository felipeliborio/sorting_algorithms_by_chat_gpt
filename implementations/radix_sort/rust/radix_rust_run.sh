if ! [[ -f ./radix_sort.app ]] || test $2 = "-c"
then
    # rustc ./radix_sort.rs -o ./radix_sort.app -C opt-level=2
    cargo build -r
fi

if [[ -f $1.radix_sort.out.rust.txt ]]
then
    rm $1.radix_sort.out.rust.txt
fi

# time ./radix_sort.app $1 >> $1.radix_sort.time.txt
\time -v -a -f "Program: %C\nTotal time: %E\nUser Mode (s) %U\nKernel Mode (s) %S\nCPU: %P" cargo run -r $1s