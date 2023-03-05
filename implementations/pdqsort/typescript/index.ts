import { quickSort } from "./pdqsort"

const arr = process.argv.slice(2).map((item) => parseInt(item))
const sortedArr = quickSort(arr)
process.stdout.write("sorted "+sortedArr.join(" "))
