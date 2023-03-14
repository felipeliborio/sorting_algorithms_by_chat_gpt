if ! [ -f ./dist/pdqsort.js ] || test $2 = "-c"
then
    npm run build
fi

if [[ -f $1.pdqsort.out.typescript.txt ]]
then
    rm $1.pdqsort.out.typescript.txt
fi

time node ./dist/pdqsort.js $1 >> $1.pdqsort.time.txt
