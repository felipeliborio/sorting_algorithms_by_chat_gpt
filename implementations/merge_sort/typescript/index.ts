import { mergeSort } from "./MergeSort"

const arr = process.argv.slice(2).map((item) => parseInt(item))
const sortedArr = mergeSort(arr)
process.stdout.write("sorted "+sortedArr.join(" "))
