import * as fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf-8')

part1(input)
part2(input)

function part1(data: string) {
  const sum = Array.from(data.matchAll(/mul\(\d+,\d+\)/g))
    .map((v) => v[0])
    .reduce((acc, cur) => acc + parseMul(cur), 0)

  console.log(`part1: ${sum}`)
}

function part2(data: string) {
  const res = Array.from(data.matchAll(/mul\(\d+,\d+\)|don't\(\)|do\(\)/g))
    .map((v) => v[0])
    .reduce<{ enabled: boolean; value: number }>(
      (acc, cur) => {
        switch (cur) {
          case `don't()`:
            return { enabled: false, value: acc.value }
          case `do()`:
            return { enabled: true, value: acc.value }
          default:
            return {
              enabled: acc.enabled,
              value: acc.enabled ? acc.value + parseMul(cur) : acc.value,
            }
        }
      },
      { enabled: true, value: 0 }
    )

  console.log(`part2: ${res.value}`)
}

function parseMul(value: string): number {
  const { 0: first, 1: second } = Array.from(value.matchAll(/\d+/g)).map((v) =>
    Number.parseInt(v[0])
  )

  return first * second
}
