use std::collections::VecDeque;

pub struct Pos{
    x: i32,
    y: i32,
}

pub enum Direction{
    Up,
    Down,
    Right,
    Left,
}
pub struct Snake{
    body: VecDeque<Pos>,
    current_direction: Direction,
}

impl Snake{
    pub fn new() -> Self {
        let mut new_body: VecDeque<Pos> = VecDeque::new();

        let starting_pos_x = 6;

        for i in 0..4{
            new_body.push_back(Pos{x: starting_pos_x-i, y: 10});
        }

        Self{
            body: new_body,
            current_direction: Direction::Right,
        }

    }
}