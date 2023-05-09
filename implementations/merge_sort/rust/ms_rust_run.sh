if ! [[ -f ./merge_sort.app ]] || test $2 = "-c"
then
    # rustc ./merge_sort.rs -o ./merge_sort.app -C opt-level=2
    cargo build -r
    # rustc ./merge_sort.rs -o ./merge_sort.app -C opt-level=2
fi

if [[ -f $1.merge_sort.out.rust.txt ]]
then
    rm $1.merge_sort.out.rust.txt
fi

# time ./merge_sort.app $1 >> $1.merge_sort.time.txt
\time -v -a -f "Program: %C\nTotal time: %E\nUser Mode (s) %U\nKernel Mode (s) %S\nCPU: %P" cargo run -r $1s