if ! [ -f ./dist/RadixSort.js ] || test $2 = "-c"
then
    npm install
    npm run build
fi

if [[ -f $1.radix_sort.out.typescript.txt ]]
then
    rm $1.radix_sort.out.typescript.txt
fi

# time node ./dist/RadixSort.js $1 >> $1.radix_sort.time.txt
\time -v -a -f "Program: %C\nTotal time: %E\nUser Mode (s) %U\nKernel Mode (s) %S\nCPU: %P" node ./dist/RadixSort.js $1