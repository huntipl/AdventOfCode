package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
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

	times := strToIntArray(lines[0])
	distances := strToIntArray(lines[1])

	result := 1
	gameScore := 0

	for i := 0; i < len(distances); i++ {
		gameScore = 0
		for t := 0; t < times[i]; t++ {
			// test case
			speed := t
			timeLeft := times[i] - t
			dist := speed * timeLeft
			if dist > distances[i] {
				gameScore++
			}
		}
		result *= gameScore
	}

	println(result)

	// part 2
	result = 0
	timesRaw := strings.Fields(lines[0])
	timesRaw = timesRaw[1:]
	timeRaw := fmt.Sprintf("%v%v%v%v", timesRaw[0], timesRaw[1], timesRaw[2], timesRaw[3])
	distancesRaw := strings.Fields(lines[1])
	distancesRaw = distancesRaw[1:]
	distanceRaw := fmt.Sprintf("%v%v%v%v", distancesRaw[0], distancesRaw[1], distancesRaw[2], distancesRaw[3])

	time, err := strconv.Atoi(timeRaw)
	if err != nil {
		panic(err)
	}
	distance, err := strconv.Atoi(distanceRaw)
	if err != nil {
		panic(err)
	}

	for t := 0; t < time; t++ {
		// test case
		speed := t
		timeLeft := time - t
		dist := speed * timeLeft
		if dist > distance {
			result++
		}
	}
	println(result)
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
