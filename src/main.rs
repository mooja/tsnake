use tsnake::*;

fn main() {
    let mut snake = Snake::new(-3, -3, Direction::North);
    for _ in 0..5 {
        snake.grow();
        println!("{}\n\n", snake);
    }

    snake.change_head_direction(Turn::Right);

    snake.grow();
    snake.grow();
        println!("{}\n\n", snake);

    for _ in 0..5 {
        snake.advance();
        println!("{}\n\n", snake);
    }
}
