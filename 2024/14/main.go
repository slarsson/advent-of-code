package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type vec struct {
	x int
	y int
}

type quadrant struct {
	min vec
	max vec
}

type state struct {
	pos vec
	vel vec
}

func (s *state) Move(xEndIndex int, yEndIndex int) {
	s.pos.x += s.vel.x
	s.pos.y += s.vel.y

	if s.pos.x < 0 {
		s.pos.x = xEndIndex + s.pos.x + 1
	} else if s.pos.x > xEndIndex {
		s.pos.x = s.pos.x - xEndIndex - 1
	}

	if s.pos.y < 0 {
		s.pos.y = yEndIndex + s.pos.y + 1
	} else if s.pos.y > yEndIndex {
		s.pos.y = s.pos.y - yEndIndex - 1
	}
}

func (s *state) IsInsideQuadrant(q quadrant) bool {
	return s.pos.x >= q.min.x && s.pos.x <= q.max.x && s.pos.y >= q.min.y && s.pos.y <= q.max.y
}

func main() {
	var robots []state

	captureLines("input.txt", func(v string) {
		re := regexp.MustCompile(`-?\d+`)
		numbers := re.FindAllString(v, -1)

		x1, _ := strconv.Atoi(numbers[0])
		y1, _ := strconv.Atoi(numbers[1])
		x2, _ := strconv.Atoi(numbers[2])
		y2, _ := strconv.Atoi(numbers[3])

		robots = append(robots, state{
			pos: vec{x1, y1},
			vel: vec{x2, y2},
		})
	})

	fmt.Println("part1:", part1(clone(robots), 101, 103))
	fmt.Println("part2:", part2(clone(robots), 101, 103))
}

func part1(robots []state, width int, height int) int {
	xEndIndex := width - 1
	yEndIndex := height - 1

	for sec := 0; sec < 100; sec++ {
		for i, robot := range robots {
			robot.Move(xEndIndex, yEndIndex)
			robots[i] = robot
		}

	}

	sum := 1
	for _, area := range quadrants(width, height) {
		var count int
		for _, robot := range robots {
			if robot.IsInsideQuadrant(area) {
				count++
			}
		}

		sum *= count
	}

	return sum
}

func quadrants(width int, height int) []quadrant {
	if width%2 == 0 || height%2 == 0 {
		panic("expected odd grid size :(")
	}

	qw := width / 2
	qh := height / 2

	var out []quadrant
	for _, min := range []vec{
		{0, 0},
		{0, qh + 1},
		{qw + 1, 0},
		{qw + 1, qh + 1},
	} {
		out = append(out, quadrant{
			min: min,
			max: vec{min.x + qw - 1, min.y + qh - 1},
		})
	}

	return out
}

func part2(robots []state, width int, height int) int {
	xEndIndex := width - 1
	yEndIndex := height - 1

	sec := 0
	for {
		sec++

		currentState := map[vec]struct{}{}

		for i, robot := range robots {
			robot.Move(xEndIndex, yEndIndex)
			robots[i] = robot

			currentState[robot.pos] = struct{}{}
		}

		for pos := range currentState {
			if hasVerticalLine(pos.x, pos.y, 30, currentState) {
				return sec
			}
		}
	}
}

func hasVerticalLine(x int, y int, lineSize int, robots map[vec]struct{}) bool {
	for y2 := y; y2 < y+lineSize; y2++ {
		if _, ok := robots[vec{x, y2}]; !ok {
			return false
		}
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

func clone(arr []state) []state {
	out := make([]state, len(arr))
	copy(out, arr)
	return out
}

// func print(robots []state, width int, height int) {
// 	for y := 0; y < height; y++ {
// 		row := ""

// 		for x := 0; x < width; x++ {
// 			if slices.ContainsFunc(robots, func(robot state) bool {
// 				return robot.pos.x == x && robot.pos.y == y
// 			}) {
// 				row += "x"
// 			} else {
// 				row += "."
// 			}
// 		}

// 		fmt.Println(row)
// 	}
// }
