import { quicksortStable } from "./QuicksortStable"

const arr = process.argv.slice(2).map((item) => parseInt(item))
const sortedArr = quicksortStable(arr)
process.stdout.write("sorted "+sortedArr.join(" "))
