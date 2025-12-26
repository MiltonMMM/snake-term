use std::thread::spawn;
use crate::snake::{Direction, Pos, Snake};
use crossterm::style::{Color, SetForegroundColor, ResetColor};
use rand::Rng;

pub struct Game{
    pub is_running: bool,
    snake: Snake,
    w: i32, // weight
    h: i32, // height
    score: i32,
    apple: Pos, // position of the apple
}

impl Game{
    pub fn new() -> Self{
        let width = 60;
        let height = 25;

        let mut game = Self {
            is_running: true,
            snake: Snake::new(),
            w: width,
            h: height,
            score: 0,
            apple: Pos { x: 0, y: 0 }, // placeholder
        };

        game.apple = game.spawn_apple();
        game
    }

    pub fn spawn_apple(&self) -> Pos{
        let mut rng = rand::rng();
        loop{
            let pos = Pos {
                x: rng.random_range(0..self.w),
                y: rng.random_range(0..self.h),
            };

            if !self.snake.is_at(pos.x, pos.y) { 
                return pos;
            }
        }
    }

    pub fn check_collisions(&mut self){
        if self.snake.has_self_collision(){
            self.is_running = false;
        }else if self.is_apple_eaten(){
            let mut rng = rand::rng();
            self.score +=1;
            self.apple = Pos {
            x: rng.random_range(0..self.w),
            y: rng.random_range(0..self.h),
        }
        }
    }

    pub fn is_apple_eaten(&self) -> bool{
        if let Some(head) = self.snake.body.front() {
            head.x == self.apple.x && head.y == self.apple.y
        } else {
            false
        }
    }

    pub fn advance_tick(&mut self){
        self.snake.move_snake();
        self.check_collisions();
        self.render();

    }

    pub fn is_apple_at(&self, x: i32, y: i32) -> bool {
        self.apple.x == x && self.apple.y == y
    }

    pub fn set_direction(&mut self, new_direction: Direction){
        // check it doesnt try to do invalid movement
        if self.snake.current_direction == Direction::Up && new_direction == Direction::Down{
            return;
        }else if self.snake.current_direction == Direction::Down && new_direction == Direction::Up{
            return;
        }else if self.snake.current_direction == Direction::Right && new_direction == Direction::Left{
            return;
        }else if self.snake.current_direction == Direction::Left && new_direction == Direction::Right{
            return;
        }
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
            print!("{}", SetForegroundColor(Color::DarkGrey));
            print!("█"); 
            print!("{}", ResetColor);

            for x in 0..self.w{
                if self.snake.is_at(x, y){
                    print!("{}", SetForegroundColor(Color::Green));
                    print!("O");
                    print!("{}", ResetColor);
                }else if self.is_apple_at(x, y){
                    print!("{}", SetForegroundColor(Color::Red));
                    print!("@");
                    print!("{}", ResetColor);
                }
                else{
                    print!(" ");
                }
            }
            
            // right border
            print!("{}", SetForegroundColor(Color::DarkGrey));
            print!("█");
            print!("{}", ResetColor);
            print!("\r\n");
    
        
        }

        // bottom border
        print!("{}", SetForegroundColor(Color::DarkGrey));
        for _ in 0..self.w+2 {
            print!("█");
        }
        print!("{}", ResetColor);
        print!("\r\n");
    }
}

