if ! [ -f ./stable/dist/QuicksortUnstable.js ] || test $2 = "-c"
then
    cd ./unstable
    npm run build
    cd ..
fi

if [[ -f $1.quicksort_unstable.out.typescript.txt ]]
then
    rm $1.quicksort_unstable.out.typescript.txt
fi

time node ./stable/dist/QuicksortUnstable.js $1 >> $1.quicksort_unstable.time.txt
