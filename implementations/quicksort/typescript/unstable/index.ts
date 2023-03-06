import { quicksortUnstable } from "./QuicksortUnstable"

const arr = process.argv.slice(2).map((item) => parseInt(item))
quicksortUnstable(arr)
process.stdout.write("sorted "+arr.join(" "))
