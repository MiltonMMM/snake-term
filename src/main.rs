use std::io::{stdout, Write};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub mod snake;
pub mod game;

use game::Game;
use snake::Snake;


fn main() -> std::io::Result<()>{
    enable_raw_mode()?;

    let mut stdout = stdout();

    let game= Game::new();
    

    disable_raw_mode();
    Ok(())
}
