use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn helper(hgt:&str) -> bool {
  let t = &hgt[hgt.len()-2..hgt.len()];
  if t == "cm"{
    if hgt[0..hgt.len()-2].parse::<i32>().unwrap() >= 150 && hgt[0..hgt.len()-2].parse::<i32>().unwrap() <= 193 {
      return true;
    }
    else {
      return false;
    }
  }
  else if t == "in" {
    if hgt[0..hgt.len()-2].parse::<i32>().unwrap() >= 59 && hgt[0..hgt.len()-2].parse::<i32>().unwrap() <= 76 {
      return true;
    }
    else {
      return false;
    }
  }
  else {
    return false;
  }
}

fn main() {
  let f = File::open("./src/input.txt").unwrap();
  let reader = BufReader::new(f);

  let mut byr:bool = false;
  let mut iyr:bool = false;
  let mut eyr:bool = false;
  let mut hgt:bool = false;
  let mut hcl:bool = false;
  let mut ecl:bool = false;
  let mut pid:bool = false;

  let mut byr2:bool = false;
  let mut iyr2:bool = false;
  let mut eyr2:bool = false;
  let mut hgt2:bool = false;
  let mut hcl2:bool = false;
  let mut ecl2:bool = false;
  let mut pid2:bool = false;

  let mut count = 0;
  let mut count2 = 0;

  for line in reader.lines(){
    let line = line.unwrap();
    if line == "" {
      byr = false;
      iyr = false;
      eyr = false;
      hgt = false;
      hcl = false;
      ecl = false;
      pid = false;
      byr2 = false;
      iyr2 = false;
      eyr2 = false;
      hgt2 = false;
      hcl2 = false;
      ecl2 = false;
      pid2 = false;
      continue;
    }

    let line:Vec<&str> = line.split(" ").collect();
    let ecls = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for field in line{
      let field:Vec<&str> = field.split(":").collect();

      match field[0].as_ref() {
        "byr" => byr = true,
        "iyr" => iyr = true,
        "eyr" => eyr = true,
        "hgt" => hgt = true,
        "hcl" => hcl = true,
        "ecl" => ecl = true,
        "pid" => pid = true,
        _ => ()
      }
      
      match field[0].as_ref() {
        "byr" => {if field[1].parse::<i32>().unwrap() >= 1920 && field[1].parse::<i32>().unwrap() <= 2002 {byr2 = true}},
        "iyr" => {if field[1].parse::<i32>().unwrap() >= 2010 && field[1].parse::<i32>().unwrap() <= 2020 {iyr2 = true}},
        "eyr" => {if field[1].parse::<i32>().unwrap() >= 2020 && field[1].parse::<i32>().unwrap() <= 2030 {eyr2 = true}},
        "hgt" => {if helper(field[1]) {hgt2 = true}},
        "hcl" => {if field[1].chars().nth(0).unwrap() == '#' && field[1].len() == 7 && field[1][1..6].chars().all(char::is_alphanumeric) {hcl2 = true}},
        "ecl" => {if ecls.iter().any(|&i| i == field[1]) {ecl2 = true}},
        "pid" => {if field[1].len() == 9 && field[1].chars().all(char::is_numeric) {pid2 = true}},
        _ => ()
      }
    }

    if byr && iyr && eyr && hgt && hcl && ecl && pid {
      count += 1;
      byr = false;
      iyr = false;
      eyr = false;
      hgt = false;
      hcl = false;
      ecl = false;
      pid = false;
    }

    if byr2 && iyr2 && eyr2 && hgt2 && hcl2 && ecl2 && pid2 {
      count2 += 1;
      byr2 = false;
      iyr2 = false;
      eyr2 = false;
      hgt2 = false;
      hcl2 = false;
      ecl2 = false;
      pid2 = false;
    }

  }

  println!("Part One: {:?}", count);
  println!("Part Two: {:?}", count2);
}
