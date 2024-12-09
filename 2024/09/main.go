package main

import (
	"fmt"
	"log"
	"os"
	"slices"
)

func main() {
	input := getLine("input.txt")

	var numbers []int
	for _, r := range input {
		v := int(r - '0')
		numbers = append(numbers, v)
	}

	fmt.Println("part1:", part1(numbers))
	fmt.Println("part2:", part2(numbers))
}

type block struct {
	id     int
	offset int
	length int
}

func part1(numbers []int) int {
	blocks := buildBlocks(numbers)

	head := 0
	tail := len(blocks) - 1

	for {
		if head > tail {
			break
		}

		b1 := blocks[head]
		if b1.id != -1 {
			head++
			continue
		}

		b2 := blocks[tail]
		if b2.id == -1 {
			blocks = blocks[:tail]
			tail--
			continue
		}

		switch {
		case b1.length == b2.length:
			b1.id = b2.id

			blocks[head] = b1
			blocks = blocks[:tail]

			tail--
			head++

		case b2.length > b1.length:
			b1.id = b2.id
			b2.length -= b1.length

			blocks[head] = b1
			blocks[tail] = b2

			head++

		case b1.length > b2.length:
			b2.offset = b1.offset

			nb := block{
				id:     -1,
				length: b1.length - b2.length,
				offset: b1.offset + b2.length,
			}

			blocks = blocks[:tail]
			blocks[head] = b2
			blocks = slices.Insert(blocks, head+1, nb)

			head++
			tail--
		}
	}

	var sum int
	for _, b := range blocks {
		for i := 0; i < b.length; i++ {
			sum += b.id * (b.offset + i)
		}
	}

	return sum
}

func part2(numbers []int) int {
	blocks := buildBlocks(numbers)

	tail := len(blocks) - 1

	for {
		if tail == 0 {
			break
		}

		cur := blocks[tail]
		if cur.id == -1 {
			tail--
			continue
		}

		for i := 0; i < tail; i++ {
			b := blocks[i]

			if b.id != -1 {
				continue
			}

			if cur.length > b.length {
				continue
			}

			switch {
			case cur.length == b.length:
				b.id = cur.id
				cur.id = -1

				blocks[i] = b
				blocks[tail] = cur

			case b.length > cur.length:
				nb := block{
					id:     cur.id,
					length: cur.length,
					offset: b.offset,
				}

				b.offset += cur.length
				b.length -= cur.length

				cur.id = -1

				blocks[i] = b
				blocks[tail] = cur
				blocks = slices.Insert(blocks, i, nb)
			}

			break
		}

		tail--
	}

	var sum int
	for _, b := range blocks {
		for i := 0; i < b.length; i++ {
			if b.id == -1 {
				continue
			}

			sum += b.id * (b.offset + i)
		}
	}

	return sum
}

func buildBlocks(numbers []int) []block {
	offset := 0
	id := 0
	blocks := []block{}

	for i := 0; i < len(numbers); i += 2 {
		if i+1 == len(numbers) {
			blocks = append(blocks, block{
				id:     id,
				offset: offset,
				length: numbers[i],
			})
			break
		}

		blocks = append(blocks, block{
			id:     id,
			offset: offset,
			length: numbers[i],
		})

		offset += numbers[i]

		blocks = append(blocks, block{
			id:     -1,
			offset: offset,
			length: numbers[i+1],
		})

		id++
		offset += numbers[i+1]
	}

	return blocks
}

func getLine(path string) string {
	bytez, err := os.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}

	return string(bytez)
}
