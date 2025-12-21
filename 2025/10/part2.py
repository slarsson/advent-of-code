from z3 import Sum, Int, Optimize, sat
import re


def solve(target, buttons):
    variables = [Int(f"x{i+1}") for (i, _) in enumerate(buttons)]

    solver = Optimize()

    for i, value in enumerate(target):
        eq = []
        for j, button in enumerate(buttons):
            if button[i] == 1:
                eq.append(Int(f"x{j+1}"))
        solver.add(Sum(eq) == value)

    for var in variables:
        solver.add(var >= 0)

    solver.minimize(Sum(variables))

    if solver.check() == sat:
        model = solver.model()
        return model.evaluate(Sum(variables)).as_long()
    else:
        raise Exception("no soluation")


def parse():
    out = []

    with open("./input.txt") as f:
        for row in f:
            target = [int(v) for v in re.findall(r"\{(.*?)\}", row)[0].split(",")]
            buttons = []

            for res in re.findall(r"\((.*?)\)", row):
                button = [0 for _ in target]
                for index in [int(v) for v in res.split(",")]:
                    button[index] = 1
                buttons.append(button)

            out.append((target, buttons))

    return out


if __name__ == "__main__":
    sum = 0
    for target, button in parse():
        sum += solve(target, button)
    print("part2:", sum)
