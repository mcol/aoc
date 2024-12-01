package main

import "core:fmt"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

day01 :: proc() {
	input := "data/input-01.txt"

	// read the data file
	data, ok := os.read_entire_file(input, context.allocator)
	if !ok {
		fmt.println("Error in reading file")
		return
	}
	defer delete(data, context.allocator)

	col1: [dynamic]int
	col2: [dynamic]int
	defer delete(col1)
	defer delete(col2)

	// parse each line in the file
	it := string(data)
	for line in strings.split_lines_iterator(&it) {
		ss := strings.split(line, "   ")
		append(&col1, strconv.atoi(ss[0]))
		append(&col2, strconv.atoi(ss[1]))
	}

	// sort the arrays
	slice.sort(col1[:])
	slice.sort(col2[:])

	// compute the distance
	dist := 0
	for val, idx in col1 {
		dist += abs(val - col2[idx])
	}
	fmt.println("Part 1:", dist) // 2815556

	// largest value encountered
	max := max(slice.last(col1[:]), slice.last(col2[:]))

	// counts for each value in col2
	counts := make([]int, max + 1)
	defer delete(counts)
	for val in col2 {
		counts[val] += 1
	}

	// compute similarity
	sim := 0
	for val in col1 {
		sim += val * counts[val]
	}
	fmt.println("Part 2:", sim) // 23927637
}
