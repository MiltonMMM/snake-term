use crate::snake::Snake;
use crossterm::style::{Color, SetForegroundColor, ResetColor};

pub struct Game{
    is_running: bool,
    snake: Snake,
    w: i32, // weight
    h: i32, // height
    score: i32,
}

impl Game{
    pub fn new() -> Self{
        Self{
            is_running:false,
            snake: Snake::new(),
            w: 20,
            h: 20,
            score: 0,
        }
    }

    pub fn advance_tick(&mut self){

        self.snake.move_snake();
        self.render();

    }

    pub fn render(&self){

        // top border
        print!("{}", SetForegroundColor(Color::DarkGrey));
        print!("█");
        for _ in 0..self.w+1 {
            print!("█");
        }
        print!("\r\n");

        for y in 0..self.h{
            // left border
            print!("█"); 
            print!("{}", ResetColor);

            for x in 0..self.w{
                if self.snake.is_at(x, y){
                    print!("■");
                }
                else{
                    print!(" ");
                }
            }
            
            // right border
            print!("{}", SetForegroundColor(Color::DarkGrey));
            print!("█"); 
            print!("\r\n");
        
        }

        // bottom border
        print!("█");
        for _ in 0..self.w+1 {
            print!("█");
        }
    }
}

