use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("./src/input.txt").unwrap();
  let reader = BufReader::new(f);

  let mut part_one_count = 0;
  let mut part_two_count = 0;

  for line in reader.lines(){
    let line = line.unwrap();
    let split:Vec<&str> = line.split(" ").collect();
    let nums:Vec<&str> = split[0].split("-").collect();

    let min = nums[0].parse::<usize>().unwrap();
    let max = nums[1].parse::<usize>().unwrap();
    let letter = split[1].chars().next().unwrap();
    let password = split.last().unwrap();

    let count = password.matches(letter).count();

    if count >= min && count <= max {
      part_one_count += 1;
    }

    let fbool:bool = password.chars().nth(min-1).unwrap() == letter;
    let sbool:bool = password.chars().nth(max-1).unwrap() == letter;

    if fbool ^ sbool {
      part_two_count += 1;
    }
    
  }
  println!("Part One: {:?}",part_one_count);
  println!("Part Two: {:?}",part_two_count);
}
