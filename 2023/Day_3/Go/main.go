package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	total := 0
	totalGears := 0

	// go over lines
	for li, line := range lines {
		start := 0
		end := 0

		// go over characters
		for i, s := range line {
			if unicode.IsDigit(s) {
				end = i + 1
			}
			if !unicode.IsDigit(s) || (unicode.IsDigit(s) && i == len(line)-1) {
				if end > start { // here is the end of the number
					raw := line[start:end]
					number, _ := strconv.Atoi(raw)

					// test for surrounding
					above := ""
					mid := ""
					below := ""
					cs := start
					ce := end

					if start-1 >= 0 {
						cs = start - 1
					}
					if end < len(line) {
						ce = end + 1
					}
					mid = line[cs:start] + line[end:ce]
					if li-1 >= 0 {
						above = lines[li-1][cs:ce]
					}
					if li+1 < len(lines) {
						below = lines[li+1][cs:ce]
					}

					surround := above + mid + below

					surround = strings.ReplaceAll(surround, ".", "")

					if len(surround) > 0 {
						total += number
					}
				}
				start = i + 1
			}
		}

		// PART 2
		previous := ""
		current := line
		next := ""

		if li-1 >= 0 {
			previous = lines[li-1]
		}
		if li+1 < len(lines) {
			next = lines[li+1]
		}

		pNums := getPrevNums(previous)
		cNums := getPrevNums(current)
		nNums := getPrevNums(next)
		allNums := append(pNums, append(cNums, nNums...)...)

		// gears
		for i, s := range line {
			var subs []int
			if s == '*' { //gear
				// prev numbers
				for _, pNum := range allNums {
					if (i <= pNum.end && pNum.end <= i+2) || (i <= pNum.start && pNum.start <= i+1) {
						subs = append(subs, pNum.value)
					}
				}
			}
			if len(subs) == 2 {
				totalGears += multiply(subs)
			}
		}
	}
	println(total, " ", totalGears)
}

func getPrevNums(line string) []Number {
	var nums []Number

	start := 0
	end := 0

	for i, s := range line {
		if unicode.IsDigit(s) {
			end = i + 1
		}
		if !unicode.IsDigit(s) || (unicode.IsDigit(s) && i == len(line)-1) {
			if end > start { // here is the end of the number
				raw := line[start:end]
				number, _ := strconv.Atoi(raw)

				nums = append(nums, Number{value: number, start: start, end: end})
			}
			start = i + 1
		}
	}
	return nums
}

type Number struct {
	value int
	start int
	end   int
}

func multiply(nums []int) int {
	result := 1
	for _, num := range nums {
		result *= num
	}
	return result
}
