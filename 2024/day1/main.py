
INPUT = """
3   4
4   3
2   5
1   3
3   9
3   3
"""

EXPECTED_RESULT = 11


def parse_input(input: str) -> tuple[list[int],list[int]]:
    list1 = []
    list2 = []
    for line in input.split("\n"):
        if line == "":
            continue
        line = line.replace("   ", " ")
        num1, num2 = line.split(" ")
        list1.append(int(num1))
        list2.append(int(num2))
    return list1, list2

def get_diffs(list1: list[int], list2: list[int]) -> list[int]:
    return [abs(list1[i] - list2[i]) for i in range(len(list1))]

def get_similarity(list1: list[int], list2: list[int]) -> int:
    res = 0
    for l1 in list1:
        for l2 in list2:
            if l1 == l2:
                res += l1
    return res


def main():
    input = open("input.txt").read()
    list1, list2 = parse_input(input)
    list1 = sorted(list1)
    list2 = sorted(list2)
    diffs = get_diffs(list1, list2)
    print(sum(diffs))
    print(get_similarity(list1, list2))

if __name__ == "__main__":
    main()


