def parse_input(file: str) -> tuple[list[int], list[int]]:
    # File format: int space int
    with open(file) as f:
        l1, l2 = [], []
        for line in f.readlines():
            int1, int2 = line.split()
            l1.append(int(int1))
            l2.append(int(int2))
    return l1, l2

def day1(l1: list[int], l2: list[int]):
    l1_s = sorted(l1)
    l2_s = sorted(l2)

    sum_of_dists = sum([abs(l1_s[i] - l2_s[i]) for i in range(len(l1))])

    print(sum_of_dists)

    sim_score = 0
    for num in l1:
        sim_score += num * l2.count(num)
    print(sim_score)

if __name__ == "__main__":
    day1(*parse_input("input.txt"))