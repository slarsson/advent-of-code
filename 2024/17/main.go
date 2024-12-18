package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"regexp"
	"strconv"
)

func main() {
	data, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	re := regexp.MustCompile(`\d+`)
	numbers := re.FindAllString(string(data), -1)

	reg := [3]int{}
	prog := []int{}

	for i, n := range numbers {
		v, err := strconv.Atoi(n)
		if err != nil {
			log.Fatal(err)
		}

		if i < 3 {
			reg[i] = v
		} else {
			prog = append(prog, v)
		}

	}

	fmt.Println("part1:", toString(solve(reg, prog)))
}

func toString(arr []int) string {
	var str string
	for i, v := range arr {
		str += strconv.Itoa(v)
		if i < len(arr)-1 {
			str += ","
		}
	}
	return str
}

func solve(reg [3]int, prog []int) []int {
	cursor := 0

	var out []int

	for {
		if cursor >= len(prog) {
			break
		}

		opcode := prog[cursor]
		operand := prog[cursor+1]

		//fmt.Println(cursor, opcode, operand, out)

		switch opcode {
		case 0:
			a := float64(reg[0])
			b := math.Pow(2, float64(comboOp(reg, operand)))
			reg[0] = int(a / b)

		case 1:
			reg[1] = reg[1] ^ operand

		case 2:
			reg[1] = comboOp(reg, operand) % 8

		case 3:
			if reg[0] != 0 {
				cursor = operand
				continue
			}

		case 4:
			reg[1] = reg[1] ^ reg[2]

		case 5:
			out = append(out, comboOp(reg, operand)%8)

		case 6:
			a := float64(reg[0])
			b := math.Pow(2, float64(comboOp(reg, operand)))
			reg[1] = int(a / b)

		case 7:
			a := float64(reg[0])
			b := math.Pow(2, float64(comboOp(reg, operand)))
			reg[2] = int(a / b)

		default:
			panic("noop")
		}

		cursor += 2
	}

	return out
}

func comboOp(reg [3]int, v int) int {
	switch v {
	case 0, 1, 2, 3:
		return v
	case 4:
		return reg[0]
	case 5:
		return reg[1]
	case 6:
		return reg[2]
	default:
		panic("noop")
	}
}
