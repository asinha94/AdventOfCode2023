from typing import Dict, List, Tuple
import timeit

def part1():
    running_total = 0
    for line in open('puzz1.txt'):
        nums = []
        for c in line:
            try:
                num = int(c)
                nums.append(num)
            except ValueError:
                continue
        
        if not nums:
            continue

        num = (nums[0]*10) + nums[-1]
        running_total += num

    print(running_total)

def part2():
    numbers = dict(one=1,
                   two=2,
                   three=3,
                   four=4,
                   five=5,
                   six=6,
                   seven=7,
                   eight=8,
                   nine=9)
    
    numbers_prefix: Dict[str, List[Tuple[str, int]]] = {}
    for number, num in numbers.items():
        c = number[0] 
        if c not in numbers_prefix:
            numbers_prefix[c] = []
        numbers_prefix[c].append((number, num))

    for num_list in numbers_prefix.values():
        num_list.sort(key=lambda x: len(x[0]))
    
    running_total = 0
    for line in open('puzz1.txt'):
        nums = []

        i = 0
        while i < len(line):
            c = line[i]
            try:
                num = int(c)
                nums.append(num)
                i += 1
                continue
            except ValueError:
                pass

            if c not in numbers_prefix:
                i += 1
                continue

            prefixes = numbers_prefix[c]
            for number, num in prefixes:
                j = i + len(number)
                if line[i:j] == number:
                    # we can use last letter of number as start of new number
                    i = j-1
                    nums.append(num)
                    break
            else:
                i += 1

        num = (nums[0]*10) + nums[-1]
        running_total += num
        #print(f'{line.strip()}, {nums}, {num}')

    print(running_total)


part2()

