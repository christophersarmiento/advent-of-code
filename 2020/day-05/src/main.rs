use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn find_row_col(directions:&Vec<char>) -> (i32, i32) {
  let mut row = 0;
  let mut col = 0;

  let mut left = 0;
  let mut right = 127;
  for direction in &directions[0..7] {
    let diff = right - left;
    if diff == 1 {
      match direction {
        'F' => {
          row = left;
          break;
        },
        'B' => {
          row = right;
          break;
        },
        _ => {}
      } 
    }
    match direction {
      'F' => {
        let res:f32 = (diff / 2) as f32;
        right = left + res.floor() as i32;
      },
      'B' => {
        let res:f32 = (diff / 2) as f32;
        left = left + res.ceil() as i32 + 1;
      },
      _ => {}
    } 
  }

  let mut left = 0;
  let mut right = 7;
  for direction in &directions[7..10] {
    let diff = right - left;
    if diff == 1 {
      match direction {
        'L' => {
          col = left;
          break;
        },
        'R' => {
          col = right;
          break;
        },
        _ => {}
      } 
    }
    match direction {
      'L' => {
        let res:f32 = (diff / 2) as f32;
        right = left + res.floor() as i32;
      },
      'R' => {
        let res:f32 = (diff / 2) as f32;
        left = left + res.ceil() as i32 + 1;
      },
      _ => {}
    } 
  }
  return (row, col);
}

fn main() {
  let f = File::open("./src/input.txt").unwrap();
  let reader = BufReader::new(f);

  let mut seats = Vec::new();

    for line in reader.lines(){
      let line = line.unwrap();
      let line:Vec<char> = line.chars().collect();
      let rowcol = find_row_col(&line);
      let row = rowcol.0;
      let col = rowcol.1;
      let id = row * 8 + col;
      seats.push(id);
    }

  println!("Part One: {:?}", seats.iter().max().unwrap());

  for seat in &seats {
    let next = seat + 1;
    if !&seats.contains(&next) && seat != seats.iter().max().unwrap(){
      println!("Part Two: {:?}", seat + 1);
    }
  }
}
