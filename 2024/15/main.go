package main

import (
	"fmt"
	"log"
	"os"
	"slices"
	"strings"
)

type vec struct {
	x int
	y int
}

func main() {
	data, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	sections := strings.Split(string(data), "\n\n")

	width := len(strings.Split(sections[0], "\n")[0])

	boxes := []vec{}
	walls := map[vec]struct{}{}
	start := vec{}

	x := 0
	y := 0

	for _, r := range sections[0] {
		if !(r == '.' || r == '#' || r == '@' || r == 'O') {
			continue
		}

		switch r {
		case '#':
			walls[vec{x, y}] = struct{}{}
		case 'O':
			boxes = append(boxes, vec{x, y})
		case '@':
			start = vec{x, y}
		}

		x++
		if x == width {
			x = 0
			y++
		}

	}

	var moves []vec
	for _, r := range sections[1] {
		switch r {
		case '<':
			moves = append(moves, vec{-1, 0})
		case '^':
			moves = append(moves, vec{0, -1})
		case 'v':
			moves = append(moves, vec{0, 1})
		case '>':
			moves = append(moves, vec{1, 0})
		}
	}

	fmt.Println("part1:", part1(moves, start, boxes, walls))
	fmt.Println("part2:", part2(moves, start, boxes, walls))
}

func part1(moves []vec, start vec, boxArr []vec, walls map[vec]struct{}) int {
	boxes := map[vec]struct{}{}
	for _, box := range boxArr {
		boxes[box] = struct{}{}
	}

	cur := start

	for _, move := range moves {
		next := cur
		next.x += move.x
		next.y += move.y

		// if wall -> do nothing
		if _, ok := walls[next]; ok {
			continue
		}

		// if empty space -> move
		if _, ok := boxes[next]; !ok {
			cur = next
			continue
		}

		box := next
		boxesToMove := []vec{box}
		spaceExists := true
		for {
			box.x += move.x
			box.y += move.y

			if _, ok := walls[box]; ok {
				spaceExists = false
				break
			}

			if _, ok := boxes[box]; ok {
				boxesToMove = append(boxesToMove, box)
				continue
			}

			break
		}

		if spaceExists {
			cur = next

			for _, box := range boxesToMove {
				delete(boxes, box)
			}

			for _, box := range boxesToMove {
				boxes[vec{box.x + move.x, box.y + move.y}] = struct{}{}
			}
		}

	}

	var sum int
	for box := range boxes {
		sum += box.y*100 + box.x
	}

	return sum
}

func part2(moves []vec, start vec, boxArr []vec, walls map[vec]struct{}) int {
	start.x *= 2

	wideWalls := map[vec]struct{}{}
	for wall := range walls {
		wideWalls[vec{x: wall.x * 2, y: wall.y}] = struct{}{}
		wideWalls[vec{x: wall.x*2 + 1, y: wall.y}] = struct{}{}
	}

	var boxes []vec
	for _, box := range boxArr {
		boxes = append(boxes, vec{x: box.x * 2, y: box.y})
	}

	cur := start

	for _, move := range moves {
		next := cur
		next.x += move.x
		next.y += move.y

		if _, ok := wideWalls[next]; ok {
			continue
		}

		if wideBoxExists(next, boxes) == nil {
			cur = next
			continue
		}

		boxesToMove := map[vec]struct{}{}
		canMove := canMoveWideBoxes(next, move, wideWalls, boxes, boxesToMove)

		if canMove {
			cur = next

			for boxToRemove := range boxesToMove {
				boxes = slices.DeleteFunc(boxes, func(v vec) bool {
					return boxToRemove.x == v.x && boxToRemove.y == v.y
				})
			}

			for b := range boxesToMove {
				b.x += move.x
				b.y += move.y

				boxes = append(boxes, b)
			}
		}
	}

	var sum int
	for _, box := range boxes {
		sum += box.y*100 + box.x
	}

	return sum
}

func wideBoxExists(pos vec, boxes []vec) *vec {
	for _, box := range boxes {
		if box.y != pos.y {
			continue
		}

		if box.x == pos.x || box.x+1 == pos.x {
			return &box
		}
	}

	return nil
}

func canMoveWideBoxes(pos vec, delta vec, walls map[vec]struct{}, boxes []vec, moves map[vec]struct{}) bool {
	box := wideBoxExists(pos, boxes)
	if box == nil {
		_, ok := walls[pos]
		return !ok
	}

	if _, ok := moves[*box]; ok {
		return true
	}

	moves[*box] = struct{}{}

	switch {
	case delta.x == 1:
		next := *box
		next.x += 2

		return canMoveWideBoxes(next, delta, walls, boxes, moves)

	case delta.x == -1:
		next := *box
		next.x -= 1

		return canMoveWideBoxes(next, delta, walls, boxes, moves)

	case delta.y != 0:
		next1 := *box
		next1.y += delta.y

		next2 := *box
		next2.x += 1
		next2.y += delta.y

		first := canMoveWideBoxes(next1, delta, walls, boxes, moves)
		second := canMoveWideBoxes(next2, delta, walls, boxes, moves)

		return first && second
	}

	panic(":(")
}

// func print(walls map[vec]struct{}, boxes map[vec]struct{}, size int, cur vec) {

// 	fmt.Println("================")
// 	for y := 0; y < size/2; y++ {
// 		row := ""

// 		for x := 0; x < size; x++ {
// 			if _, ok := walls[vec{x, y}]; ok {
// 				row += "#"
// 			} else if _, ok := boxes[vec{x, y}]; ok {
// 				row += "[]"
// 				x++
// 			} else if cur.x == x && cur.y == y {
// 				row += "@"
// 			} else {
// 				row += "."
// 			}
// 		}

// 		fmt.Println(row)
// 	}
// 	fmt.Println("================")

// }
