use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("./src/input.txt").unwrap();
  let reader = BufReader::new(f);

  for line in reader.lines(){
    let line = line.unwrap();

  }

  println!("Part One: {:?}", ());
  println!("Part Two: {:?}", ());
}
