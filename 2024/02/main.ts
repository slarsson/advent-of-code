import * as fs from 'node:fs'

const input = fs
  .readFileSync('input.txt', 'utf-8')
  .split('\n')
  .filter((v) => v != '')
  .map((row) =>
    row
      .split(' ')
      .filter((v) => v != '')
      .map((v) => Number.parseInt(v))
  )

part1(input)
part2(input)

function part1(levels: number[][]) {
  const res = levels.reduce((acc, cur) => {
    if (validate(cur)) {
      return acc + 1
    }
    return acc
  }, 0)

  console.log(`part1: ${res}`)
}

function part2(levels: number[][]) {
  const res = levels.reduce((acc, cur) => {
    if (validate(cur)) {
      return acc + 1
    }

    for (let i = 0; i < cur.length; i++) {
      const arr = [...cur].filter((_, index) => index != i)

      if (validate(arr)) {
        return acc + 1
      }
    }

    return acc
  }, 0)

  console.log(`part2: ${res}`)
}

function validate(arr: number[]): boolean {
  if (arr.length <= 1) {
    return true
  }

  let prev = 0
  for (let i = 0; i < arr.length - 1; i++) {
    const diff = arr[i] - arr[i + 1]

    if (Math.abs(diff) > 3) {
      return false
    }

    if (diff == 0) {
      return false
    }

    if (diff < 0 && prev > 0) {
      return false
    }

    if (diff > 0 && prev < 0) {
      return false
    }

    prev = diff
  }

  return true
}
