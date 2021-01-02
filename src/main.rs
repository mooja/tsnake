extern crate termion;

mod game;
mod snake;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::io::{stdout, stdin};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use game::GameEvent;


// mod snake;

fn main() {
    let (tx, rx): (mpsc::Sender<GameEvent>, mpsc::Receiver<GameEvent>) = mpsc::channel();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    let tx1 = tx.clone();
    let _tick_thread = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(200));
            tx1.send(GameEvent::Tick).unwrap();
        }
    });

    let tx2 = tx.clone();
    let _input_thread = thread::spawn(move || {
        for c in stdin.keys() {
            match c.unwrap() {
                x => tx2.send(GameEvent::KeyPress(x)).unwrap(),
            }
        }
    });

    let mut game = game::Game::new(rx, stdout);
    game.run();
}