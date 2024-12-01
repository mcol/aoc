package main

import "core:fmt"
import "core:os"
import "core:strconv"

main :: proc() {
	args := os.args
	if len(args) == 1 {
		fmt.println("Usage: odin run odin -- <day to solve>")
		return
	}
	day := strconv.atoi(args[1])
	switch day {
	case 1:
		day01()
	case 1 ..< 25:
		fmt.println("Day", day, "not solved yet")
	case:
		fmt.println("Day must be between 1 and 25")
	}
}
