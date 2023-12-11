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

	result := 0
	resultBak := 0

	data := make([]int, 0)

	for _, line := range lines {
		data = strToIntArray(line)
		// solution := data[len(data)-1]
		// data = data[:len(data)-1]
		subData := make([][]int, 0)
		depth := 0
		subData = append(subData, data)
		for {
			subData = append(subData, make([]int, 0))

			for i := 0; i < len(subData[depth])-1; i++ {
				tmp := subData[depth][i+1] - subData[depth][i]
				subData[depth+1] = append(subData[depth+1], tmp)
			}
			if sumAbs(subData[depth+1]) != 0 {
				depth += 1
			} else {
				// println("TODO got to the bottom of it, ", depth, result)
				// printSubData(subData)
				for i := depth; i >= 0; i-- {

					// println(subData[i][len(subData[i])-1])
					last := subData[i][len(subData[i])-1]
					lastPrev := subData[i+1][len(subData[i+1])-1]
					next := last + lastPrev
					subData[i] = append(subData[i], next)

					// Part 2
					first := subData[i][0]
					lastFirst := subData[i+1][0]
					previous := first - lastFirst
					subData[i] = append([]int{previous}, subData[i]...)
				}
				// res := subData[0][len(subData[0])-1]
				// if res != solution {
				// 	println("WRONG ANSWER. EXPECTED", solution, "GOT", res)
				// }
				result += subData[0][len(subData[0])-1]
				resultBak += subData[0][0]
				// println(subData[0][len(subData[0])-1], "--------------")
				break
			}
			if depth == len(data)-1 {
				// unable to find pattern
				println("Not possible")
				break
			}
		}
	}
	println(result)
	println(resultBak)
}

func printSubData(subData [][]int) {
	spaces := 0
	for _, v := range subData {
		for i := 0; i < spaces; i++ {
			print(" ")
		}
		for _, vv := range v {
			print(vv, " ")
		}
		print("\n")
		spaces += 1
	}
}

func strToIntArray(str string) []int {
	vals := strings.Fields(str)
	var res []int
	for _, v := range vals {
		val, _ := strconv.Atoi(v)
		res = append(res, val)
	}
	return res
}

func sumAbs(input []int) int {
	result := 0
	for _, v := range input {
		result += abs(v)
	}
	return result
}

func abs(v int) int {
	if v < 0 {
		return v * -1
	}
	return v
}
