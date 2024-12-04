package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type coord struct {
	x int
	y int
}

func main() {
	matrix := map[coord]string{}
	x := 0

	captureLines("input.txt", func(v string) {
		for y, char := range strings.Split(v, "") {
			matrix[coord{x, y}] = char
		}
		x++
	})

	fmt.Println("part1:", part1(matrix))
	fmt.Println("part2:", part2(matrix))
}

func part1(matrix map[coord]string) int {
	var sum int
	for c := range matrix {
		sum += solve(c.x, c.y, matrix)
	}

	return sum / 2
}

func part2(matrix map[coord]string) int {
	var sum int
	for c := range matrix {
		if kernel(c.x, c.y, matrix) {
			sum++
		}
	}

	return sum
}

func kernel(x int, y int, matrix map[coord]string) bool {
	leftToRight := matrix[coord{x - 1, y - 1}] + matrix[coord{x, y}] + matrix[coord{x + 1, y + 1}]
	rightToLeft := matrix[coord{x - 1, y + 1}] + matrix[coord{x, y}] + matrix[coord{x + 1, y - 1}]
	return (leftToRight == "MAS" || leftToRight == "SAM") && (rightToLeft == "MAS" || rightToLeft == "SAM")
}

func solve(xStart int, yStart int, matrix map[coord]string) int {
	var sum int

	for _, delta := range []coord{
		{0, 1},
		{0, -1},
		{1, 0},
		{-1, 0},
		{1, -1},
		{1, 1},
		{-1, -1},
		{-1, 1},
	} {

		i := 0
		x := xStart
		y := yStart
		word := ""

		for {
			v, ok := matrix[coord{x, y}]
			if !ok {
				break
			}

			word = word + v

			if word == "XMAS" || word == "SAMX" {
				sum++
				break
			}

			if len(word) == 4 {
				break
			}

			if !(strings.HasPrefix("XMAS", word) || strings.HasPrefix("SAMX", word)) {
				break
			}

			i++
			if i == 4 {
				break
			}

			x += delta.x
			y += delta.y
		}
	}

	return sum
}

func captureLines(path string, f func(v string)) {
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		f(scanner.Text())
	}
}
