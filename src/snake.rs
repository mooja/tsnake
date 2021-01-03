#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
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
    pub fn new() -> Self {
        let p = Part {
            position: Position { x: 0, y: 0 },
            direction: Direction::North,
        };

        Snake { parts: vec![p] }
    }

    pub fn get_head_position(&self) -> Position {
        self.parts[0].position
    }

    pub fn get_positions(&self) -> Vec<Position> {
        self.parts.iter().map(|p| p.position).collect()
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

    pub fn change_head_direction(&mut self, direction: Direction) {
        let head_direction = self.parts[0].direction;
        if head_direction == direction || direction == head_direction.opposite() {
            return;
        }
        self.parts.insert(
            0,
            Part {
                position: self.parts[0].position,
                direction: direction,
            },
        );
        self.parts.remove(1);
    }
}
