import * as fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf-8').split('\n\n')

part1()
part2()

function part1() {
  const before = getRules()
  const lists = getLists()

  let sum = 0

  for (const list of lists) {
    if (isOrdered(list, before)) {
      if (list.length % 2 == 0) throw 'even array length'
      sum += list[(list.length - 1) / 2]
    }
  }

  console.log(`part1: ${sum}`)
}

function part2() {
  const before = getRules()
  const lists = getLists()

  let sum = 0

  for (const list of lists) {
    if (isOrdered(list, before)) {
      continue
    }

    const ordered = reorder(list, before)

    if (ordered.length % 2 == 0) throw 'even array length'
    sum += ordered[(ordered.length - 1) / 2]
  }

  console.log(`part2: ${sum}`)
}

function reorder(list: number[], beforeRules: Map<number, number[]>): number[] {
  const sortedList = [...list].sort((a, b) => {
    const before = beforeRules.get(a)

    if (before?.includes(b)) {
      return -1
    }

    return 1
  })

  return sortedList
}

function isOrdered(
  list: number[],
  beforeRules: Map<number, number[]>
): boolean {
  for (let i = 0; i < list.length; i++) {
    const cur = list[i]
    const head = list.slice(0, i)

    const beforeValues = beforeRules.get(cur)

    if (head.find((element, index) => beforeValues?.includes(element))) {
      return false
    }
  }

  return true
}

function getRules() {
  let before = new Map<number, number[]>()

  for (const row of input[0].split('\n')) {
    const { 0: first, 1: second } = row
      .split('|')
      .map((v) => Number.parseInt(v))

    const currentBefore = before.get(first)
    before.set(first, currentBefore ? [...currentBefore, second] : [second])
  }

  return before
}

function getLists(): number[][] {
  return input[1]
    .split('\n')
    .filter((row) => row != '')
    .map((row) => {
      return row.split(',').map((num) => Number.parseInt(num))
    })
}
