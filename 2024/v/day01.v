module day01

import arrays
import math
import os
import strconv

pub fn day01() {
	input := 'data/input-01.txt'

	// parse each line in the file
	file := os.read_lines(input) or { panic('Could not read file') }
	mut col1 := []int{}
	mut col2 := []int{}
	for line in file {
		val1, val2 := line.split_once('   ') or { panic('') }
		col1 = arrays.concat(col1, strconv.atoi(val1) or { panic('') })
		col2 = arrays.concat(col2, strconv.atoi(val2) or { panic('') })
	}

	// sort the arrays
	col1.sort()
	col2.sort()

	// compute the distance
	mut dist := 0
	for idx, val1 in col1 {
		dist += math.abs(val1 - col2[idx])
	}
	println('Part 1: ${dist}') // 2815556

	// largest value encountered
	max := math.max(col1.last(), col2.last())

	// counts for each value in col2
	mut counts := []int{len: max + 1}
	for val in col2 {
		counts[val] += 1
	}

	// compute the similarity
	mut sim := 0
	for val in col1 {
		sim += val * counts[val]
	}

	println('Part 2: ${sim}') // 23927637
}
