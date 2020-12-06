use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
  let f = File::open("./src/input.txt").unwrap();
  let reader = BufReader::new(f);

  let mut totals:Vec<i32> = Vec::new();
  let mut allyes:Vec<i32> = Vec::new();

  let mut group_size:i32 = 0;
  let mut group_counts = HashMap::<char,i32>::new();

  for line in reader.lines(){
    let line = line.unwrap();

    if line == "" {
      totals.push(group_counts.keys().count() as i32);
      allyes.push(group_counts.values().filter(|&i| i >= &group_size).into_iter().count() as i32);

      group_size = 0;
      group_counts.clear();
      continue;
    }

    group_size += 1;
    line.chars().for_each(|i| *(group_counts.entry(i).or_insert(0)) += 1);

  }

  totals.push(group_counts.keys().count() as i32);
  allyes.push(group_counts.values().filter(|&i| i >= &group_size).into_iter().count() as i32);


  println!("Part One: {:?}", totals.iter().sum::<i32>());
  println!("Part Two: {:?}", allyes.iter().sum::<i32>());
}
