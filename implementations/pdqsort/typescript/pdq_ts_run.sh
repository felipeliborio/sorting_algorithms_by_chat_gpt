if ! [ -f ./dist/pdqsort.js ] || test $2 = "-c"
then
    npm install
    npm run build
fi

if [[ -f $1.pdqsort.out.typescript.txt ]]
then
    rm $1.pdqsort.out.typescript.txt
fi

# time node ./dist/pdqsort.js $1 >> $1.pdqsort.time.txt
\time -v -a -f "Program: %C\nTotal time: %E\nUser Mode (s) %U\nKernel Mode (s) %S\nCPU: %P" node ./dist/pdqsort.js $1
