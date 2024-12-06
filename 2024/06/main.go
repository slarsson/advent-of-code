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
	var start coord
	var width int
	var height int

	obstacles := map[coord]struct{}{}

	y := 0
	captureLines("input.txt", func(v string) {
		width = len(v)
		for x, value := range strings.Split(v, "") {
			switch value {
			case "#":
				obstacles[coord{x: x, y: y}] = struct{}{}
			case "^":
				start.x = x
				start.y = y
			}
		}

		y++
	})

	height = y

	fmt.Println("part1:", part1(start, obstacles, width, height))
	fmt.Println("part2:", part2(start, obstacles, width, height))

}

func part1(start coord, obstacles map[coord]struct{}, width int, height int) int {
	visited := map[coord]struct{}{}
	visited[start] = struct{}{}

	dir := coord{0, -1}
	cur := start

	for {
		pos, newDir, completed := nextWithRotate90(cur, dir, obstacles, width, height)
		if completed {
			break
		}

		visited[pos] = struct{}{}
		cur = pos
		dir = newDir
	}

	return len(visited)
}

func part2(start coord, obstacles map[coord]struct{}, width int, height int) int {
	var cycles int
	var prev *coord

	for x := 0; x < width; x++ {
		for y := 0; y < height; y++ {
			obstacle := coord{x, y}

			if _, ok := obstacles[obstacle]; ok {
				continue
			}

			if obstacle == start {
				continue
			}

			if prev != nil {
				delete(obstacles, *prev)
			}
			prev = &obstacle
			obstacles[obstacle] = struct{}{}

			dir := coord{0, -1}
			cur := start

			visited := map[coord]coord{}

			for {
				pos, newDir, completed := nextWithRotate90(cur, dir, obstacles, width, height)
				if completed {
					break
				}

				if prevDir, ok := visited[pos]; ok {
					if newDir == prevDir {
						cycles++
						break
					}
				}

				cur = pos
				dir = newDir

				visited[pos] = dir
			}

		}
	}

	return cycles
}

func nextWithRotate90(cur coord, dir coord, obstacles map[coord]struct{}, width int, height int) (coord, coord, bool) {
	for {
		next := coord{x: cur.x + dir.x, y: cur.y + dir.y}

		if next.x < 0 || next.x >= width {
			return coord{}, coord{}, true
		}

		if next.y < 0 || next.y >= height {
			return coord{}, coord{}, true
		}

		if _, ok := obstacles[next]; ok {
			dir = rotate90(dir)
			continue
		}

		return next, dir, false
	}
}

func rotate90(c coord) coord {
	// x = x * cos + y * sin => -y
	// y = -x * sin + y cos => x
	return coord{-c.y, c.x}
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
