if ! [ -f ./stable/dist/QuicksortStable.js ] || test $2 = "-c"
then
    cd ./stable
    npm run build
    cd ..
fi

if [[ -f $1.quicksort_stable.out.typescript.txt ]]
then
    rm $1.quicksort_stable.out.typescript.txt
fi

time node ./stable/dist/QuicksortStable.js $1 >> $1.quicksort_stable.time.txt
