use quicksilver::{
    geom::{Rectangle, Shape},
};

const ACCEL: f32 = 1.5;
const JUMP: f32 = 26.0;

pub struct Player{
    pub hitbox: Rectangle,
    pub vel: f32,
}

impl Player{
    pub fn new(x: f32, y: f32) -> Self{
        Player{
            hitbox: Rectangle::new((x, y), (25.0, 25.0)),
            vel: 0.0,
        }
    }

    pub fn update(&mut self){
        self.hitbox = self.hitbox.translate((0.0, self.vel + ACCEL/2.0));
        self.vel += ACCEL;
    }

    pub fn jump(&mut self){
        self.vel -= JUMP;
    }
}
