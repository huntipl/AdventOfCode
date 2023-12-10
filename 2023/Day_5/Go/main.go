package main

import (
	"bufio"
	"fmt"
	"os"
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

	// go over lines
	almanac := buildAlmanac(lines)

	println("Built almanac")

	// Part 1
	lowest := -1
	for i := 0; i < len(almanac.seeds); i++ {
		// check seeds and update lowest here
		min := processSeed(almanac.seeds[i], almanac)
		if lowest == -1 {
			lowest = min
		} else {
			lowest = minaAB(min, lowest)
		}
	}
	print("Part 1: ")
	println(lowest)

	// Part 2
	lowest = -1

	start := 0
	count := 0

	for i := 0; i < len(almanac.seedRanges); i++ {
		start = almanac.seedRanges[i].start
		count = almanac.seedRanges[i].count
		for j := 0; j < count; j++ {
			// check seeds and update lowest here
			min := processSeed(start+j, almanac)
			if lowest == -1 {
				lowest = min
			} else {
				lowest = minaAB(min, lowest)
			}
		}

		println("Batch done")
	}
	print("Part 2: ")
	println(lowest)

}

func processSeed(seed int, almanac Almanac) int {
	results := []int{seed}
	results = mapAlm(results, almanac.seedToSoil)
	results = mapAlm(results, almanac.soilToFertilizer)
	results = mapAlm(results, almanac.fertilizerToWater)
	results = mapAlm(results, almanac.waterToLight)
	results = mapAlm(results, almanac.lightToTemperature)
	results = mapAlm(results, almanac.temperatureToHumidity)
	results = mapAlm(results, almanac.humidityToLocation)
	return min(results)
}

func buildAlmanac(lines []string) Almanac {
	mode := Mode{}
	almanac := Almanac{}

	entry := Entry{}

	for _, line := range lines {
		switch line {
		case "":
			{
				continue
			}
		case "humidity-to-location map:":
			{
				mode.reset()
				mode.humidityToLocation = true
				continue
			}
		case "temperature-to-humidity map:":
			{
				mode.reset()
				mode.temperatureToHumidity = true
				continue
			}
		case "light-to-temperature map:":
			{
				mode.reset()
				mode.lightToTemperature = true
				continue
			}
		case "water-to-light map:":
			{
				mode.reset()
				mode.waterToLight = true
				continue
			}
		case "fertilizer-to-water map:":
			{
				mode.reset()
				mode.fertilizerToWater = true
				continue
			}
		case "soil-to-fertilizer map:":
			{
				mode.reset()
				mode.soilToFertilizer = true
				continue
			}
		case "seed-to-soil map:":
			{
				mode.reset()
				mode.seedToSoil = true
				continue
			}
		}
		if strings.HasPrefix(line, "seeds: ") {
			rawSeeds := sToAI(line[7:])
			almanac.seeds = rawSeeds
			for i := 0; i < len(rawSeeds); i += 2 {
				almanac.seedRanges = append(almanac.seedRanges, SeedRange{rawSeeds[i], rawSeeds[i+1]})
			}
		}

		split := strings.Fields(line)

		entry.destination, _ = strconv.Atoi(split[0])
		entry.source, _ = strconv.Atoi(split[1])
		entry.length, _ = strconv.Atoi(split[2])

		switch mode {
		case Mode{seedToSoil: true}:
			almanac.seedToSoil = append(almanac.seedToSoil, entry)
		case Mode{soilToFertilizer: true}:
			almanac.soilToFertilizer = append(almanac.soilToFertilizer, entry)
		case Mode{fertilizerToWater: true}:
			almanac.fertilizerToWater = append(almanac.fertilizerToWater, entry)
		case Mode{waterToLight: true}:
			almanac.waterToLight = append(almanac.waterToLight, entry)
		case Mode{lightToTemperature: true}:
			almanac.lightToTemperature = append(almanac.lightToTemperature, entry)
		case Mode{temperatureToHumidity: true}:
			almanac.temperatureToHumidity = append(almanac.temperatureToHumidity, entry)
		case Mode{humidityToLocation: true}:
			almanac.humidityToLocation = append(almanac.humidityToLocation, entry)
		default:
			continue
		}
	}
	return almanac
}

func sToAI(s string) []int {
	sArr := strings.Fields(s)
	iArr := make([]int, len(sArr))
	for i, s := range sArr {
		val, _ := strconv.Atoi(s)
		iArr[i] = val
	}
	return iArr
}

type Mode struct {
	seedToSoil            bool
	soilToFertilizer      bool
	fertilizerToWater     bool
	waterToLight          bool
	lightToTemperature    bool
	temperatureToHumidity bool
	humidityToLocation    bool
}

func (m *Mode) reset() {
	m.seedToSoil = false
	m.soilToFertilizer = false
	m.fertilizerToWater = false
	m.waterToLight = false
	m.lightToTemperature = false
	m.temperatureToHumidity = false
	m.humidityToLocation = false
}

type Almanac struct {
	seedToSoil            []Entry
	soilToFertilizer      []Entry
	fertilizerToWater     []Entry
	waterToLight          []Entry
	lightToTemperature    []Entry
	temperatureToHumidity []Entry
	humidityToLocation    []Entry
	seeds                 []int
	seedRanges            []SeedRange
}

type SeedRange struct {
	start int
	count int
}

type Entry struct {
	destination int
	source      int
	length      int
}

func (a Almanac) String() string {
	output := ""

	output += "Seed to soil map:\n"
	for _, m := range a.seedToSoil {
		output += fmt.Sprintf("%d | %d | %d\n", m.destination, m.source, m.length)
	}
	output += "-----------\n"

	output += "Soil to fertilizer map:\n"
	for _, m := range a.soilToFertilizer {
		output += fmt.Sprintf("%d | %d | %d\n", m.destination, m.source, m.length)
	}
	output += "-----------\n"

	output += "Fertilizer to water map:\n"
	for _, m := range a.fertilizerToWater {
		output += fmt.Sprintf("%d | %d | %d\n", m.destination, m.source, m.length)
	}
	output += "-----------\n"

	output += "Water to light map:\n"
	for _, m := range a.waterToLight {
		output += fmt.Sprintf("%d | %d | %d\n", m.destination, m.source, m.length)
	}
	output += "-----------\n"

	output += "Light to temperature map:\n"
	for _, m := range a.lightToTemperature {
		output += fmt.Sprintf("%d | %d | %d\n", m.destination, m.source, m.length)
	}
	output += "-----------\n"

	output += "Temperature to humidity map:\n"
	for _, m := range a.temperatureToHumidity {
		output += fmt.Sprintf("%d | %d | %d\n", m.destination, m.source, m.length)
	}
	output += "-----------\n"

	output += "Humidity to location map:\n"
	for _, m := range a.humidityToLocation {
		output += fmt.Sprintf("%d | %d | %d\n", m.destination, m.source, m.length)
	}
	output += "-----------\n"

	return output
}

func mapAlm(inputs []int, entry []Entry) []int {
	results := make([]int, 0)
	result := -1
	for _, inp := range inputs {
		// seed to soil
		result = -1
		for _, entry := range entry {
			if inp >= entry.source && inp < entry.source+entry.length {
				//in range
				diff := inp - entry.source
				result = entry.destination + diff
				break
			}
		}
		if result == -1 {
			result = inp
		}
		results = append(results, result)
	}
	return results
}

func min(inputs []int) int {
	min := inputs[0]
	for _, inp := range inputs {
		if inp < min {
			min = inp
		}
	}
	return min
}

func minaAB(a int, b int) int {
	if a < b {
		return a
	}
	return b
}
