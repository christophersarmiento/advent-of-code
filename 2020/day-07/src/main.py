def parse_contains(string:str) -> dict:
  l = {}
  string = string.split(",")
  for bag in string:
    color = bag.strip()[2:].strip()[:-4].strip()
    try:
      num = int(bag.strip().split(" ")[0])
    except ValueError:
      num = 0
    l[color]=num
  return l

def counter(d:dict, k:str) -> int:
  count = 0
  for k,v in d[k].items():
    if v == 0:
      continue
    else:
      count += v
      count += v * counter(d,k)
  return count

if __name__ == "__main__":
  bags = {}
  can_hold_shiny_gold = set([])

  with open("./input.txt") as f:
    for line in f:
      line = line.strip().strip(".").split(" bags contain ")
      bag = line[0]
      contains = parse_contains(line[1])
      bags[bag] = contains

      if "shiny gold" in contains:
        can_hold_shiny_gold.add(bag)

  j = 0
  while j < len(bags):
    for k, v in bags.items():
      if any(i in can_hold_shiny_gold for i in v.keys()) and k not in can_hold_shiny_gold:
        can_hold_shiny_gold.add(k)
        j = 0
      else:
        j += 1

  bag_to_find = "shiny gold"
  print(f"Part One: {len(can_hold_shiny_gold)}")
  print(f"Part Two: {counter(bags,bag_to_find)}")