use crate::utils::read::read_list;
use intcode_computer::prelude::*;
use crate::utils::geometry::{CardinalDirection, Vec2D};
use std::collections::HashSet;

fn get_test_input() -> IntcodeMemoryType {
    read_list(include_str!("day17_input.txt"), ",")
}

fn get_ascii(program: IntcodeMemoryType) -> Vec<char> {
    let mut computer = IntCodeComputer::new(program);
    let mut out_buffer = Vec::<char>::new(); 
    execute! {computer,
        output { out_buffer.push(computer.take_output() as u8 as char) }
    }
    while let Some('\n') = out_buffer.last() {
      out_buffer.pop();
    }
    out_buffer
}

struct View {
  data: Vec<Vec<SpaceKind>>
}

impl View {
  fn new(ascii_buf: Vec<char>) -> Self {
    let spaces: Vec<Vec<SpaceKind>> = ascii_buf.split(|c| *c == '\n')
      .map(|line| {
        line.into_iter().map(|c| {
          match c {
            '#' => SpaceKind::Scaffold,
            '.' => SpaceKind::Empty,
            '>' => SpaceKind::Robot(CardinalDirection::East),
            '<' => SpaceKind::Robot(CardinalDirection::West),
            '^' => SpaceKind::Robot(CardinalDirection::North),
            'v' => SpaceKind::Robot(CardinalDirection::South),
            _ => panic!("Unsupported char {}", c),
          }
        }).collect()
      }).collect();
    let mut len: Option<usize> = Option::None;
    for line in spaces.iter() {
      if len.is_none() {
        len = Some(line.len());
        continue;
      }
      assert_eq!(len.unwrap(), line.len());
    }
    View { data: spaces }
  }

  fn get(&self, coord: &Vec2D) -> Option<&SpaceKind> {
    self.data.get(coord.y as usize).and_then(|row| row.get(coord.x as usize))
  }

  fn find_intersections(&self) -> HashSet<Vec2D> {
    let mut intersections = HashSet::new();
    for (y, row) in self.data.iter().enumerate() {
      for (x, space) in row.iter().enumerate() {
        match space {
          SpaceKind::Empty => continue,
          _non_empty => {
            let coord = Vec2D::new(x as i32, y as i32);
            let mut scaffold_neighbors = 0;
            let neighbors = CardinalDirection::iterator()
              .map(|dir| {
                match dir {
                  CardinalDirection::North => Vec2D::new(0, -1),
                  CardinalDirection::South => Vec2D::new(0, 1),
                  CardinalDirection::East => Vec2D::new(1, 0),
                  CardinalDirection::West => Vec2D::new(-1, 0),
                }
              })
              .map(|unit| unit + coord);
            for neighbor in neighbors {
              if let Some(space) = self.get(&neighbor) {
                match space {
                  SpaceKind::Scaffold | SpaceKind::Robot(_) => scaffold_neighbors += 1,
                  SpaceKind::Empty => {}
                }
              }
            }
            if scaffold_neighbors >= 4 {
              assert_eq!(scaffold_neighbors, 4);
              intersections.insert(coord);
            }
          }
        }
      }
    }
    intersections
  }
}

impl std::fmt::Debug for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for line in self.data.iter() {
          for space in line {
            match space {
              SpaceKind::Empty => s.push('.'),
              SpaceKind::Scaffold => s.push('#'),
              SpaceKind::Robot(_d) => s.push('R'),
            }
          }
          s.push('\n');
        }
        write!(f, "{}", s)
    }
}

pub fn calculate_alignment_param() -> i32 {
  let view = View::new(get_ascii(get_test_input()));
  let intersections = view.find_intersections();
  let mut sum = 0;
  for intersection in intersections {
    sum += (intersection.x * intersection.y);
  }
  sum
}

enum SpaceKind {
  Empty,
  Scaffold,
  Robot(CardinalDirection)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_can_construct_input() {
    let input = get_test_input();
    let ascii = get_ascii(input);
    View::new(ascii);
  }

  #[test]
  fn test_can_find_intersections() {
    let view = View::new(get_ascii(get_test_input()));
    let intersections = view.find_intersections();
    assert_ne!(0, intersections.len());
  }

  #[test]
  fn test_correct_answer_part_1() {
    assert_eq!(3936, calculate_alignment_param());
  }
}
