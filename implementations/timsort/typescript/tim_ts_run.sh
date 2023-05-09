if ! [ -f ./dist/Timsort.js ] || test $2 = "-c"
then
    npm install
    npm run build
fi

if [[ -f $1.timsort.out.typescript.txt ]]
then
    rm $1.timsort.out.typescript.txt
fi

# time node ./dist/Timsort.js $1 >> $1.timsort.time.txt
\time -v -a -f "Program: %C\nTotal time: %E\nUser Mode (s) %U\nKernel Mode (s) %S\nCPU: %P" node ./dist/Timsort.js $1
