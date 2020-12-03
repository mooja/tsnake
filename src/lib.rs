use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Turn {
    Left,
    Right,
}

impl Direction {
    pub fn turned(&self, t: Turn) -> Direction {
        match t {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },

            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn advanced(&self, direction: Direction) -> Position {
        match direction {
            Direction::North => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Part {
    pub position: Position,
    pub direction: Direction,
}

#[derive(Debug)]
pub struct Snake {
    parts: Vec<Part>,
}

impl Snake {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        let p = Part {
            position: Position { x, y },
            direction: direction,
        };

        Snake { parts: vec![p] }
    }

    pub fn get_head_direction(self) -> Direction {
        self.parts[0].direction
    }

    pub fn get_head_position(&self) -> Position {
        self.parts[0].position
    }

    pub fn next_head_position(&self) -> Position {
        let dir = self.parts[0].direction;
        self.parts[0].position.advanced(dir)
    }

    pub fn advance(&mut self) {
        let mut last_direction = self.parts[0].direction;
        for p in &mut self.parts {
            let temp_direction = p.direction;
            *p = Part {
                position: p.position.advanced(p.direction),
                direction: last_direction,
            };
            last_direction = temp_direction;
        }
    }

    pub fn grow(&mut self) {
        let (position, direction) = (self.parts[0].position, self.parts[0].direction);
        let new_part = Part {
            position: position.advanced(direction),
            direction,
        };
        self.parts.insert(0, new_part);
    }

    pub fn change_head_direction(&mut self, turn: Turn) {
        self.parts.insert(
            0,
            Part {
                position: self.parts[0].position,
                direction: self.parts[0].direction.turned(turn),
            },
        );
        self.parts.remove(1);
    }
}

pub fn draw_positions(positions: Vec<Position>) -> String {
    let left = positions.iter().map(|p| p.x).min().unwrap();
    let right = positions.iter().map(|p| p.x).max().unwrap();
    let top = positions.iter().map(|p| p.y).max().unwrap();
    let bottom = positions.iter().map(|p| p.y).min().unwrap();
    let width = right - left + 1;
    let height = top - bottom + 1;

    let mut bit_grid: Vec<Vec<bool>> = Vec::new();
    for _ in 0..height {
        bit_grid.push(vec![false; width as usize]);
    }

    for pos in &positions {
        bit_grid[(pos.y - bottom) as usize][(pos.x - left) as usize] = true;
    }

    let mut line_strings: Vec<String> = bit_grid
        .iter()
        .map(|bit_vec| {
            let mut line = String::new();
            for bit in bit_vec {
                match bit {
                    true => line.push_str("#"),
                    false => line.push_str(" "),
                }
            }
            line
        })
        .collect();
    line_strings.reverse();
    line_strings.join("\n")
}

impl Display for Snake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let positions: Vec<Position> = self.parts.iter().map(|p| p.position).collect();
        let display_string = draw_positions(positions);
        write!(f, "{}", display_string)
    }
}
