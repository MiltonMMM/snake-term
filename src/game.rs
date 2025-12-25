use crate::snake::Snake;
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
}

