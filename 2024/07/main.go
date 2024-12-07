package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type eq struct {
	result int
	values []int
}

func main() {
	var equations []eq
	captureLines("input.txt", func(v string) {
		parts := strings.Split(v, ":")

		result, err := strconv.Atoi(parts[0])
		if err != nil {
			log.Fatal(err)
		}

		var values []int
		for _, str := range strings.Split(parts[1], " ") {
			if str == "" {
				continue
			}

			value, err := strconv.Atoi(str)
			if err != nil {
				log.Fatal(err)
			}

			values = append(values, value)
		}

		equations = append(equations, eq{
			result: result,
			values: values,
		})
	})

	fmt.Println("part1:", part1(equations))
	fmt.Println("part2:", part2(equations))
}

func part1(equations []eq) int {
	var sum int

	for _, eq := range equations {
		if solve1(eq.result, 0, 0, eq.values) {
			sum += eq.result
		}

	}

	return sum
}

func part2(equations []eq) int {
	var sum int

	for _, eq := range equations {
		if solve2(eq.result, 0, 0, eq.values) {
			sum += eq.result
		}

	}

	return sum
}

func solve1(target int, sum int, index int, values []int) bool {
	if index == len(values) {
		return target == sum
	}

	add := solve1(target, sum+values[index], index+1, values)

	if index == 0 {
		sum = 1
	}

	mul := solve1(target, sum*values[index], index+1, values)

	return add || mul
}

func solve2(target int, sum int, index int, values []int) bool {
	if index == len(values) {
		return target == sum
	}

	add := solve2(target, sum+values[index], index+1, values)

	cc := solve2(target, concat(sum, values[index]), index+1, values)

	if index == 0 {
		sum = 1
	}

	mul := solve2(target, sum*values[index], index+1, values)

	return add || cc || mul
}

func concat(a, b int) int {
	res, err := strconv.Atoi(fmt.Sprintf("%d%d", a, b))
	if err != nil {
		log.Fatal(err)
	}

	return res
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
