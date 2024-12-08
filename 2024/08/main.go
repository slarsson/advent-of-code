package main

import (
	"bufio"
	"fmt"
	"os"
)

type vec struct {
	x int
	y int
}

func main() {
	var width int
	antennas := map[rune][]vec{}
	y := 0
	captureLines("input.txt", func(v string) {
		width = len(v)
		for x, char := range v {
			if char == '.' {
				continue
			}

			antennas[char] = append(antennas[char], vec{x, y})
		}
		y++
	})

	height := y

	fmt.Println("part1:", part1(antennas, width, height))
	fmt.Println("part2:", part2(antennas, width, height))
}

func part1(antennas map[rune][]vec, width int, height int) int {
	antinodes := map[vec]struct{}{}

	for _, positions := range antennas {
		for i := 0; i < len(positions)-1; i++ {
			for j := i + 1; j < len(positions); j++ {
				a := positions[i]
				b := positions[j]

				ak := vec{x: a.x - b.x, y: a.y - b.y}
				bk := vec{x: -ak.x, y: -ak.y}

				aa := vec{x: a.x + ak.x, y: a.y + ak.y}
				bb := vec{x: b.x + bk.x, y: b.y + bk.y}

				if isInsideMap(width, height, aa) {
					antinodes[aa] = struct{}{}
				}

				if isInsideMap(width, height, bb) {
					antinodes[bb] = struct{}{}
				}
			}
		}
	}

	return len(antinodes)
}

func part2(antennas map[rune][]vec, width int, height int) int {
	antinodes := map[vec]struct{}{}

	for _, positions := range antennas {
		for i := 0; i < len(positions)-1; i++ {
			for j := i + 1; j < len(positions); j++ {
				a := positions[i]
				b := positions[j]

				antinodes[a] = struct{}{}

				ak := vec{x: a.x - b.x, y: a.y - b.y}

				i := 1
				for {
					aa := vec{x: a.x + i*ak.x, y: a.y + i*ak.y}
					if !isInsideMap(width, height, aa) {
						break
					}

					antinodes[aa] = struct{}{}

					i++
				}

				j := -1
				for {
					aa := vec{x: a.x + j*ak.x, y: a.y + j*ak.y}
					if !isInsideMap(width, height, aa) {
						break
					}

					antinodes[aa] = struct{}{}

					j--
				}
			}
		}
	}

	return len(antinodes)
}

func isInsideMap(width int, height int, v vec) bool {
	if v.x < 0 || v.x >= width {
		return false
	}

	if v.y < 0 || v.y >= height {
		return false
	}

	return true
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
