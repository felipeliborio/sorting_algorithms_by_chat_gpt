if ! [ -f ./dist/Timsort.js ] || test $2 = "-c"
then
    npm run build
fi

if [[ -f $1.timsort.out.typescript.txt ]]
then
    rm $1.timsort.out.typescript.txt
fi

time node ./dist/Timsort.js $1 >> $1.timsort.time.txt
