module day02

import os
import strconv

// check the difference between successive elements
fn eval_safe(levs []int) bool {
	for i in 1 .. levs.len {
		diff := levs[i] - levs[i - 1]
		if diff < 1 || diff > 3 {
			return false
		}
	}
	return true
}

pub fn day02() {
	input := 'data/input-02.txt'
	file := os.read_lines(input) or { panic('Could not read file') }
	mut safe := []bool{len: file.len}

	// parse each line in the file
	for idx, line in file {
		mut levs := line.split(' ').map(strconv.atoi(it) or { panic('') })

		// reverse the array if decreasing
		if levs[1] < levs[0] {
			levs.reverse_in_place()
		}
		safe[idx] = eval_safe(levs)
	}

	// count the safe lines
	mut num := safe.filter(it == true).len
	println('Part 1: ${num}') // 326

	// parse each line in the file
	safe = []bool{len: file.len, init: false}
	for idx, line in file {
		mut levs := line.split(' ').map(strconv.atoi(it) or { panic('') })

		// we need to check the first 3 diffs before we can decide if the
		// array is decreasing
		mut num_decr := 0
		for i in 1 .. 4 {
			if levs[i] < levs[i - 1] {
				num_decr += 1
			}
		}

		// reverse the array if decreasing
		if num_decr > 1 {
			levs.reverse_in_place()
		}

		// check if safe without requiring exceptions
		if eval_safe(levs) {
			safe[idx] = true
			continue
		}

		// check if it is safe with one exception
		for i in 0 .. levs.len {
			mut sub := levs.clone()
			sub.delete(i)
			if eval_safe(sub) {
				safe[idx] = true
				break
			}
		}
	}

	// count the safe lines
	num = safe.filter(it == true).len
	println('Part 2: ${num}') // 381
}
