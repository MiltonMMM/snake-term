use std::io::{stdout, Write};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

pub mod snake;
pub mod game;

use game::Game;
use snake::Snake;

use crate::snake::Direction;


fn main() -> std::io::Result<()>{
    enable_raw_mode()?;

    let mut stdout = stdout();

    let mut game= Game::new();

    let tick_duration = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {

        let elapsed = last_tick.elapsed();

        let timeout = tick_duration
            .checked_sub(elapsed)
            .unwrap_or(Duration::from_millis(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('w') | KeyCode::Up => game.set_direction(Direction::Up),
                    KeyCode::Char('s') | KeyCode::Down => game.set_direction(Direction::Down),
                    KeyCode::Char('d') | KeyCode::Right => game.set_direction(Direction::Right),
                    KeyCode::Char('a') | KeyCode::Left => game.set_direction(Direction::Left),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_duration {
            last_tick = Instant::now();

            execute!(
                stdout,
                Clear(ClearType::All),
                MoveTo(0, 0)
            )?;

            game.advance_tick();
            stdout.flush()?;
        }

        thread::sleep(Duration::from_millis(1));
    }


    disable_raw_mode()?;
    Ok(())
}
