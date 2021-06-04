import sys


if __name__ == "__main__":

    n = int(sys.stdin.readline().rstrip())
    sticks = list(map(int, sys.stdin.readline().split(" ")))
    sticks.sort()

    median = 0

    # if even take average
    if n % 2 == 0:
        median = (sticks[n // 2 - 1] + sticks[n // 2]) // 2

    # take middle element
    else:
        median = sticks[n // 2]

    total_cost = sum(map(lambda stick: abs(stick - median), sticks))
    print(total_cost)
