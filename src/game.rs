use crate::snake::{Direction, Snake};
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
            w: 60,
            h: 25,
            score: 0,
        }
    }

    pub fn advance_tick(&mut self){

        self.snake.move_snake();
        self.render();

    }

    pub fn set_direction(&mut self, new_direction: Direction){
        self.snake.current_direction = new_direction;
    }

    pub fn render(&self){

        // top border
        print!("{}", SetForegroundColor(Color::DarkGrey));
        for _ in 0..self.w+2 {
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
        for _ in 0..self.w+2 {
            print!("█");
        }
        print!("\r\n");
    }
}

