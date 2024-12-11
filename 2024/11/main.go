package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	const input = "890 0 1 935698 68001 3441397 7221 27"

	var stones []int64

	for _, v := range strings.Split(input, " ") {
		n, _ := strconv.ParseInt(v, 10, 64)
		stones = append(stones, n)
	}

	fmt.Println("part1:", solve(stones, 25))
	fmt.Println("part2:", solve(stones, 75))
}

func solve(stones []int64, blinks int) int64 {
	m1 := map[int64]int64{}

	for _, s := range stones {
		m1[s]++
	}

	for i := 0; i < blinks; i++ {
		m2 := map[int64]int64{}
		for k, v := range m1 {
			if k == 0 {
				m2[1] += v
			} else if a, b, ok := split(k); ok {
				m2[a] += v
				m2[b] += v
			} else {
				m2[k*2024] += v
			}
		}

		m1 = m2
	}

	var sum int64
	for _, v := range m1 {
		sum += v
	}

	return sum
}

func split(n int64) (int64, int64, bool) {
	str := fmt.Sprintf("%d", n)
	if len(str)%2 == 1 {
		return 0, 0, false
	}

	middle := len(str) / 2

	first, _ := strconv.ParseInt(str[:middle], 10, 64)
	second, _ := strconv.ParseInt(str[middle:], 10, 64)

	return first, second, true
}
