use crate::snake;

use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc;
use termion;
use std::io;
use std::io::{Write, stdout, stdin};


#[derive(Debug)]
pub enum GameEvent {
    Tick,
    KeyPress(termion::event::Key)
}

#[derive(Debug)]
enum GameState {
    StartScreen,
    Active,
}

#[derive(Debug)]
pub struct Grid {
    width: i32,
    height: i32,
}


pub struct Game {
    event_channel: mpsc::Receiver<GameEvent>,
    stdout: termion::raw::RawTerminal<io::Stdout>,
    snake: snake::Snake,
    dot: snake::Position,
    state: GameState,
    // grid: Grid,
}

impl Game {
    pub fn new(event_channel: mpsc::Receiver<GameEvent>, stdout: termion::raw::RawTerminal<io::Stdout>) -> Self {
        let snake = snake::Snake::new();
        let dot = snake::Position { x: 5, y: 5 };
        let state = GameState::Active;

        Game {
            event_channel,
            stdout,
            snake,
            dot,
            state,
        }

    }

    pub fn run(&mut self) {
        loop {
            let event = self.event_channel.recv().unwrap();
            match event {
                GameEvent::KeyPress(termion::event::Key::Char('q')) => {
                    write!(self.stdout, "{}{}", termion::clear::All, termion::cursor::Show).unwrap();
                    break;
                }

                GameEvent::KeyPress(termion::event::Key::Char(c)) => {
                    match c {
                        'w' | 'k' => self.snake.change_head_direction(snake::Direction::North),
                        's' | 'j' => self.snake.change_head_direction(snake::Direction::South),
                        'a' | 'h' => self.snake.change_head_direction(snake::Direction::West),
                        'd' | 'l' => self.snake.change_head_direction(snake::Direction::East),
                        _ => ()
                    }
                }

                GameEvent::KeyPress(termion::event::Key::Up) => self.snake.change_head_direction(snake::Direction::North),
                GameEvent::KeyPress(termion::event::Key::Left) => self.snake.change_head_direction(snake::Direction::West),
                GameEvent::KeyPress(termion::event::Key::Right) => self.snake.change_head_direction(snake::Direction::East),
                GameEvent::KeyPress(termion::event::Key::Down) => self.snake.change_head_direction(snake::Direction::South),

                GameEvent::Tick => {
                    self.snake.advance();
                    if self.snake_oob() || self.snake_in_itself() {
                        break;
                    }

                    if self.snake_on_dot() {
                        self.snake.grow();
                        self.make_new_dot();
                    }
                }

                _ => ()
            }

            self.draw();
        }
    }

    pub fn draw(&mut self) {
        let (width, height) = termion::terminal_size().unwrap();
        let x_offset = (width / 2) as i32;
        let y_offset = (height / 2) as i32;
        let snake_positions = self.snake.get_positions();
        let mut positions = snake_positions;
        positions.push(self.dot);

        write!(self.stdout, "{}", termion::clear::All).unwrap();
        for p in positions {
            write!(self.stdout, "{}#{}",
                termion::cursor::Goto((x_offset + p.x) as u16, (y_offset - p.y) as u16),
                termion::cursor::Hide
            ).unwrap();
        }
        self.stdout.flush().unwrap();
    }

    fn snake_on_dot(&self) -> bool {
        self.snake.get_positions().iter().any(|p| *p == self.dot)
    }

    fn snake_oob(&self) -> bool {
        let (width, height) = termion::terminal_size().unwrap();
        let x_offset = (width / 2) as i32;
        let y_offset = (height / 2) as i32;
        let head = &self.snake.get_head_position();
        return head.x <= -x_offset 
               || head.x >= x_offset
               || head.y <= -y_offset 
               || head.y >= y_offset
    }

    fn snake_in_itself(&self) -> bool {
        let head = &self.snake.get_head_position();
        let rest = &self.snake.get_positions()[1..];
        rest.iter().any(|p| *p == *head)
    }

    fn make_new_dot(&mut self) {
        let mut rng = rand::thread_rng();
        let (width, height) = termion::terminal_size().unwrap();
        let x_offset = (width / 2) as i32;
        let y_offset = (height / 2) as i32;
        self.dot = loop {
            let rand_pos = snake::Position {
                x: rng.gen_range(-x_offset, x_offset),
                y: rng.gen_range(-y_offset, y_offset)
            };

            if !self.snake.get_positions().contains(&rand_pos) {
                break rand_pos;
            }
        }
    }
}