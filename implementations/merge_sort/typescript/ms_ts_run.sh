if ! [ -f ./dist/MergeSort.js ] || test $2 = "-c"
then
    npm install
    npm run build
fi

if [[ -f $1.merge_sort.out.typescript.txt ]]
then
    rm $1.merge_sort.out.typescript.txt
fi

# time node ./dist/MergeSort.js $1 >> $1.merge_sort.time.txt
\time -v -a -f "Program: %C\nTotal time: %E\nUser Mode (s) %U\nKernel Mode (s) %S\nCPU: %P" node ./dist/MergeSort.js $1
