package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	var left []int
	var right []int

	captureLines("input.txt", func(v string) {
		parts := strings.Split(v, " ")

		first, err := strconv.Atoi(parts[0])
		if err != nil {
			log.Fatal(err)
		}

		second, err := strconv.Atoi(parts[3])
		if err != nil {
			log.Fatal(err)
		}

		left = append(left, first)
		right = append(right, second)
	})

	part1 := part1(clone(left), clone(right))
	part2 := part2(clone(left), clone(right))

	fmt.Println("part1:", part1)
	fmt.Println("part2:", part2)
}

func part1(lhs []int, rhs []int) int {
	slices.Sort(lhs)
	slices.Sort(rhs)

	var sum int
	for i := 0; i < len(lhs); i++ {
		diff := lhs[i] - rhs[i]
		if diff < 0 {
			diff *= -1
		}

		sum += diff
	}

	return sum
}

func part2(lhs []int, rhs []int) int {
	scores := map[int]int{}

	for _, value := range rhs {
		scores[value]++
	}

	var sum int
	for _, value := range lhs {
		sum += value * scores[value]
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

func clone(arr []int) []int {
	out := make([]int, len(arr))
	copy(out, arr)
	return out
}
