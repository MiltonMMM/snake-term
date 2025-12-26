use std::{collections::VecDeque, env::current_dir};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Pos{
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq)]
pub enum Direction{
    Up,
    Down,
    Right,
    Left,
}
pub struct Snake{
    pub body: VecDeque<Pos>,
    pub current_direction: Direction,
}

impl Snake{
    pub fn new() -> Self {
        let mut new_body: VecDeque<Pos> = VecDeque::new();

        let starting_pos_x = 6;

        for i in 0..3{
            new_body.push_back(Pos{x: starting_pos_x-i, y: 10});
        }

        Self{
            body: new_body,
            current_direction: Direction::Right,
        }

    }

    pub fn has_self_collision(&self) -> bool {
        let head = self.body.front().expect("empty snake");

        self.body
        .iter()
        .skip(1)           // skip the head
        .any(|p| p == head)

    }

    pub fn move_snake(&mut self){
        self.body.pop_back();
        let mut new_head_pos = *self.body.front().unwrap();
        match self.current_direction{
            Direction::Up => {
                new_head_pos.y -= 1;
            },
            Direction::Down =>{
                new_head_pos.y += 1;
            },
            Direction::Right =>{
                new_head_pos.x += 1;
            },
            Direction::Left =>{
                new_head_pos.x -= 1;
            },
        }
        self.body.push_front(new_head_pos);
    }

    // Returns true if some part of the snake it at those coordinates
    pub fn is_at(&self, x: i32, y: i32) -> bool {
        self.body.iter().any(|p| p.x == x && p.y == y)
    }
}