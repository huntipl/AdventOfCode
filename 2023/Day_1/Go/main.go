package main

import (
	"bufio"
	"errors"
	"os"
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

	var sum int

	for _, line := range lines {
		firstDigit, lastDigit := -1, -1
		for i := 0; i < len(line); i++ {
			test, err := getDigit(line, i)
			if err == nil {
				// println(test)
				if firstDigit == -1 {
					firstDigit = test
				}
				lastDigit = test
				continue
			}
		}
		code := firstDigit*10 + lastDigit
		sum += code
	}

	println(sum)

}

func getDigit(s string, pos int) (int, error) {
	digitsMap := map[string]int{
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
		"zero":  0,
		"0":     0,
		"1":     1,
		"2":     2,
		"3":     3,
		"4":     4,
		"5":     5,
		"6":     6,
		"7":     7,
		"8":     8,
		"9":     9,
	}
	if pos < 0 || pos >= len(s) {
		panic("out of range")
	}
	for digit, d := range digitsMap {
		if pos+len(digit) > len(s) {
			continue
		}
		slice := s[pos : pos+len(digit)]
		if slice == digit {
			return d, nil
		}
	}
	return -1, errors.New("not a digit")
}
