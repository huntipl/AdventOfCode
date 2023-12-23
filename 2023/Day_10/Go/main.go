package main

import (
	"bufio"
	"os"
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

	maze, sI, sJ := buildMaze(lines)
	mazeCopy, _, _ := buildMaze(lines)

	dist := 0

	// check where can we go from the start:
	pos := ""
	next := Position{0, 0}

	direction := ""

	// UP
	if sI > 0 {
		pos = maze[sI-1][sJ]
		// | 7 F
		if pos == "|" || pos == "7" || pos == "F" {
			next.x = sJ
			next.y = sI - 1
			direction = "U"
		}
	}

	// DOWN
	if sI+1 < len(maze) {
		pos = maze[sI+1][sJ]
		// | L J
		if pos == "|" || pos == "L" || pos == "J" {
			next.x = sJ
			next.y = sI + 1
			direction = "D"
		}
	}

	// LEFT
	if sJ > 0 {
		pos = maze[sI][sJ-1]
		// - F L
		if pos == "-" || pos == "F" || pos == "L" {
			next.x = sJ - 1
			next.y = sI
			direction = "L"
		}
	}

	// RIGHT
	if sJ+1 < len(maze[0]) {
		pos = maze[sI][sJ+1]
		// - J 7
		if pos == "-" || pos == "J" || pos == "7" {
			next.x = sJ + 1
			next.y = sI
			direction = "R"
		}
	}

	dist, _ = followMaze(maze, next, direction, false)

	println(dist / 2)

	maze = clearJunk(maze, ".")

	maze = restorePipes(maze, mazeCopy)

	maze = surround(maze)

	result := countEnclosed(maze)

	printMaze(maze)
	println(result)
}

func surround(maze [][]string) [][]string {
	for i := 0; i < len(maze); i++ {
		for j := 0; j < len(maze[i]); j++ {
			// count pipes up down left right

			if maze[i][j] != "." {
				continue
			}

			inside := true

			// ray left
			pipes := 0
			for jj := j - 1; jj >= 0; jj-- {
				if isCross(maze[i][jj]) {
					pipes++
				}
			}
			if pipes%2 != 0 {
				// maze[i][j] = "I"
			} else {
				inside = false
			}

			// Final check
			if inside {
				maze[i][j] = "I"
			} else {
				maze[i][j] = "O"
			}

		}
	}
	return maze
}

func followMaze(maze [][]string, next Position, direction string, mark bool) (int, [][]string) {
	startDirection := direction
	endDirection := direction
	solved := false
	dist := 0
	for {
		if solved {
			break
		}
		dist += 1
		pos := maze[next.y][next.x]
		if !mark {
			maze[next.y][next.x] = "#"
		}

		switch pos {
		case "|":
			{
				if direction == "U" {
					next.y--
				} else if direction == "D" {
					next.y++
				} else {
					panic("IMPOSSIBLE | " + direction)
				}
			}
		case "-":
			{
				if direction == "L" {
					next.x--
				} else if direction == "R" {
					next.x++
				} else {
					panic("IMPOSSIBLE - " + direction)
				}
			}
		case "L":
			{
				if direction == "D" {
					next.x++
					direction = "R"
				} else if direction == "L" {
					next.y--
					direction = "U"
				} else {
					panic("IMPOSSIBLE L " + direction)
				}
			}
		case "J":
			{
				if direction == "D" {
					next.x--
					direction = "L"
				} else if direction == "R" {
					next.y--
					direction = "U"
				} else {
					panic("IMPOSSIBLE J " + direction)
				}
			}
		case "7":
			{
				if direction == "U" {
					next.x--
					direction = "L"
				} else if direction == "R" {
					next.y++
					direction = "D"
				} else {
					panic("IMPOSSIBLE 7 " + direction)
				}
			}
		case "F":
			{
				if direction == "U" {
					next.x++
					direction = "R"
				} else if direction == "L" {
					next.y++
					direction = "D"
				} else {
					panic("IMPOSSIBLE F " + direction)
				}
			}
		case "S":
			{
				solved = true
				// replace S with appropriate pipe
				endDirection = direction
				missingPiece := "S"

				switch startDirection {
				case "U", "D":
					{
						if endDirection == "U" || endDirection == "D" {
							missingPiece = "|"
						}
						if endDirection == "L" {
							if startDirection == "U" {
								missingPiece = "L"
							}
							if startDirection == "D" {
								missingPiece = "7"
							}
						}
						if endDirection == "R" {
							if startDirection == "U" {
								missingPiece = "J"
							}
							if startDirection == "D" {
								missingPiece = "F"
							}
						}
					}
				case "L", "R":
					{
						if endDirection == "L" || endDirection == "R" {
							missingPiece = "-"
						}
						if endDirection == "U" {
							if startDirection == "L" {
								missingPiece = "7"
							}
							if startDirection == "R" {
								missingPiece = "F"
							}
						}
						if endDirection == "D" {
							if startDirection == "L" {
								missingPiece = "J"
							}
							if startDirection == "R" {
								missingPiece = "L"
							}
						}
					}
				}
				_ = missingPiece
				maze[next.y][next.x] = missingPiece
				break
			}
		}
	}
	return dist, maze
}

func isCross(pos string) bool {
	switch pos {
	case "|", "J", "L":
		{
			return true
		}
	}
	return false
}

func countEnclosed(maze [][]string) int {
	result := 0
	for _, line := range maze {
		for _, pos := range line {
			if pos == "I" {
				result++
			}
		}
	}

	return result
}

func restorePipes(maze [][]string, oldMaze [][]string) [][]string {
	for i, line := range maze {
		for j, point := range line {
			if point == "#" {
				maze[i][j] = oldMaze[i][j]
			}
		}
	}
	return maze
}

func clearJunk(maze [][]string, mark string) [][]string {
	for i, line := range maze {
		for j, point := range line {
			if point != "#" {
				maze[i][j] = mark
			}
		}
	}
	return maze
}

func buildMaze(lines []string) ([][]string, int, int) {
	h, l := 0, 0
	h = len(lines)
	l = len(lines[0])
	maze := make([][]string, h)

	sI, sJ := 0, 0

	for i := range maze {
		maze[i] = make([]string, l)
	}

	for i, line := range lines {
		for j, point := range line {
			if point == 'S' {
				// println("Found start!", i, j)
				sI, sJ = i, j
			}
			maze[i][j] = string(point)
		}
	}
	return maze, sI, sJ
}

func printMaze(maze [][]string) {
	for _, line := range maze {
		for _, point := range line {
			print(string(point))
		}
		print("\n")
	}
}

type Position struct {
	x int
	y int
}
