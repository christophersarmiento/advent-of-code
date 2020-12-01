use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("./src/input.txt").unwrap();
  let reader = BufReader::new(f);

  let mut nums = Vec::new();

  for line in reader.lines(){
    let num = line.unwrap().parse::<i32>().unwrap();
    let diff = 2020 - num;
    nums.push(num);

    if nums.contains(&diff) {
      println!("Part One: {}", num * diff);
    }
  }

  nums.sort();

  for (i, num) in nums.iter().enumerate() {
    let mut left = i + 1;
    let mut right = nums.len() - 1;

    while left < right{
      let sum = num + nums[left] + nums[right];

      if sum < 2020 {
        left += 1;
      }
      else if sum > 2020 {
        right -= 1;
      }
      else {
        println!("Part Two: {}", num * nums[left] * nums[right]);
        return;
      }
    }
  }
}
