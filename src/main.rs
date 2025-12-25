use std::io::{stdout, Write};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::thread;
use std::time::Duration;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

pub mod snake;
pub mod game;

use game::Game;
use snake::Snake;


fn main() -> std::io::Result<()>{
    enable_raw_mode()?;

    let mut stdout = stdout();

    let mut game= Game::new();

    loop {

        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        game.advance_tick();
        stdout.flush()?;
        thread::sleep(Duration::from_millis(1000));

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        } else {
            thread::sleep(Duration::from_millis(200));
        }

    }


    disable_raw_mode()?;
    Ok(())
}
