package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.real")
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

	steps := make([]string, 0)
	maze := make(map[string][]string)
	starts := make(Stack, 0)

	for i, line := range lines {
		if i == 0 {
			steps = strings.Split(line, "")
			continue
		} else if line == "" {
			continue
		}

		raw := strings.Split(line, " = ")
		key := raw[0]
		raw = strings.Split(raw[1], ", ")
		maze[key] = []string{raw[0][1:], raw[1][:3]}
		if key[2:] == "A" {
			starts = append(starts, key)
		}
	}

	// Part 1
	start := "AAA"
	end := "ZZZ"
	count := 0
	pos := 0
	options := make([]string, 0)
	direction := ""
	if maze[start] == nil {
		println("Skipping Part 1")
		goto SKIP1
	}
	for {
		if pos >= len(steps) {
			pos = 0
		}
		options = maze[start]
		direction = steps[pos]
		if direction == "L" {
			start = options[0]
		} else {
			start = options[1]
		}
		if start == end {
			break
		}
		count++
		pos++
	}

	println(count + 1)

	// Part 2
SKIP1:
	count = 0
	pos = 0

	solutions := make([]int, len(starts))

	for {
		if pos >= len(steps) {
			pos = 0
		}

		for i, start := range starts {
			options = maze[start]
			direction = steps[pos]

			if direction == "L" {
				start = options[0]
			} else {
				start = options[1]
			}

			starts[i] = start
			if start[2:] == "Z" && solutions[i] == 0 {
				solutions[i] = count + 1
				println("Entry", i, "solved in", solutions[i], "steps", start)
			}
		}
		count++
		pos++

		allSolved := true
		for _, v := range solutions {
			if v == 0 {
				allSolved = false
				break
			}
		}
		if allSolved {
			for i, v := range solutions {
				println("Solution:", i, v)
			}
			println("----------")
			final := lcmL(solutions)
			println("LCM:", final)
			break
		}
	}
}

func strToIntArray(str string) []int {
	vals := strings.Fields(str)
	var res []int
	for i, v := range vals {
		if i == 0 {
			continue
		}
		val, _ := strconv.Atoi(v)
		res = append(res, val)
	}
	return res
}

type Stack []string

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func lcm(a, b int) int {
	return a / gcd(a, b) * b
}

func lcmL(nums []int) int {
	result := nums[0]
	for _, num := range nums[1:] {
		result = lcm(result, num)
	}
	return result
}
