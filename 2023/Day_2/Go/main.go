package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

var RED = 12
var GREEN = 13
var BLUE = 14

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

	sum := 0
	sum_power := 0

	for _, line := range lines {
		possible := true
		raw := strings.SplitN(line, ":", 2)
		game := strings.Split(strings.TrimSpace(raw[0]), " ")[1]
		runs := strings.Split(strings.TrimSpace(raw[1]), ";")

		min_red := 0
		min_blue := 0
		min_green := 0

		for _, run := range runs {
			series := strings.Split(strings.TrimSpace(run), ", ")
			for _, s := range series {
				count, _ := strconv.Atoi(strings.Split(strings.TrimSpace(s), " ")[0])
				color := strings.Split(strings.TrimSpace(s), " ")[1]
				// Part 1
				if color == "red" && count > RED {
					possible = false
				}
				if color == "blue" && count > BLUE {
					possible = false
				}
				if color == "green" && count > GREEN {
					possible = false
				}
				// Part 2
				if color == "red" {
					if count > min_red {
						min_red = count
					}
				}
				if color == "blue" {
					if count > min_blue {
						min_blue = count
					}
				}
				if color == "green" {
					if count > min_green {
						min_green = count
					}
				}
			}
		}
		if possible {
			game, _ := strconv.Atoi(game)
			sum += game
		}

		power := min_red * min_blue * min_green
		sum_power += power
	}

	println(sum, sum_power)
}
