module main

import day01
import os
import strconv

fn main() {
	args := os.args
	if args.len == 1 {
		println('Usage: v run v <day to solve>')
		return
	}
	day := strconv.atoi(args[1]) or { panic('positive integer required') }
	match day {
		1 { day01.day01() }
		2...25 { println('Day ${day} not solved yet') }
		else { println('Day must be between 1 and 25') }
	}
}
