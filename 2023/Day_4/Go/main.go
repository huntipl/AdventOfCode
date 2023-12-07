package main

import (
	"bufio"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("test.txt")
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
	lookup := make(map[int]int)
	cardCount := make(map[int]int)

	// go over lines
	for i, line := range lines {
		// split on :
		count := 0

		parts := strings.Split(line, ":")
		parts = strings.Split(parts[1], "|")

		winners := strToIntArr(parts[0])
		picks := strToIntArr(parts[1])

		common := make(map[int]int)

		for _, el := range winners {
			common[el]++
		}
		for _, el := range picks {
			common[el]++
		}

		for _, v := range common {
			if v == 2 {
				count++
			}
		}
		geo := int(1 * math.Pow(2, float64(count-1)))
		total += geo

		// PART 2
		lookup[i+1] = count
		cardCount[i+1]++
	}
	println(total)

	// PART 2
	total = 0

	keys := make([]int, 0, len(cardCount))
	for k := range cardCount {
		keys = append(keys, k)
	}
	sort.Ints(keys)

	for _, k := range keys {
		won := lookup[k]

		copies := cardCount[k]
		for ii := 0; ii < copies; ii++ {
			for i := k + 1; i < k+1+won; i++ {
				cardCount[i]++
			}
		}
	}

	for _, k := range keys {
		total += cardCount[k]
	}
	println(total)

}

func strToIntArr(str string) []int {
	strArr := strings.Split(str, " ")
	intArr := make([]int, len(strArr))
	for i, strNum := range strArr {
		num, _ := strconv.Atoi(strNum)
		intArr[i] = num
	}
	return intArr
}
