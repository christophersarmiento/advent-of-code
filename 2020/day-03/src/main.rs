use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("./src/input.txt").unwrap();
  let reader = BufReader::new(f);

  let mut map = Vec::new();

  for line in reader.lines(){
    let line2 : Vec<char> = line.unwrap().repeat(1000).chars().into_iter().collect();
    map.push(line2);
  }

  let mut x = 0;
  let mut y = 0;
  let mut count = 0;

  while y < map.len() {
    if map[y][x] == '#' {count += 1};
    x += 3;
    y += 1;
  }
  println!("Part One: {:?}", count);

  let mut count_r1d1 = 0;
  let mut count_r5d1 = 0;
  let mut count_r7d1 = 0;
  let mut count_r1d2 = 0;

  x = 0;
  y = 0;
  while y < map.len() {
    if map[y][x] == '#' {count_r1d1 += 1};
    x += 1;
    y += 1;
  }

  x = 0;
  y = 0;
  while y < map.len() {
    if map[y][x] == '#' {count_r5d1 += 1};
    x += 5;
    y += 1;
  }

  x = 0;
  y = 0;
  while y < map.len() {
    if map[y][x] == '#' {count_r7d1 += 1};
    x += 7;
    y += 1;
  }

  x = 0;
  y = 0;
  while y < map.len() {
    if map[y][x] == '#' {count_r1d2 += 1};
    x += 1;
    y += 2;
  }

    // println!("Right 1, Down 1: {:?}",count_r1d1);
    // println!("Right 3, Down 1: {:?}",count);
    // println!("Right 5, Down 1: {:?}",count_r5d1);
    // println!("Right 7, Down 1: {:?}",count_r7d1);
    // println!("Right 1, Down 2: {:?}",count_r1d2);

  let answer:i64 = count_r1d1 * count * count_r5d1 * count_r7d1 * count_r1d2;
  println!("Part Two: {:?}", answer);
}
