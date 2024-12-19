package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	data, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	blocks := strings.Split(string(data), "\n\n")

	var patterns []string
	for _, p := range strings.Split(blocks[0], ",") {
		patterns = append(patterns, strings.TrimSpace(p))
	}

	var designs []string
	for _, d := range strings.Split(blocks[1], "\n") {
		if d == "" {
			continue
		}

		designs = append(designs, d)
	}

	var a int
	var b int

	for _, d := range designs {
		res := solve(d, patterns, map[string]int{})

		b += res

		if res > 0 {
			a++
		}
	}

	fmt.Println("part1:", a)
	fmt.Println("part1:", b)
}

func solve(cur string, patterns []string, suffixCache map[string]int) int {
	if len(cur) == 0 {
		return 1
	}

	if count, ok := suffixCache[cur]; ok {
		return count
	}

	var sum int

	for _, p := range patterns {
		if !strings.HasPrefix(cur, p) {
			continue
		}

		sum += solve(strings.TrimPrefix(cur, p), patterns, suffixCache)
	}

	suffixCache[cur] = sum

	return sum
}
