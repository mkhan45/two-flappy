#[macro_use]
use stdweb;
use stdweb::js;

use quicksilver::{
    geom::{Rectangle, Shape},
};

use rand;

const SPEED: f32 = 10.0;
const VSPEED: f32 = -10.0;
const WIDTH: f32 = 100.0;
const INIT_GAP: f32 = 200.0 + 300.0;
const GAP_ACCEL: f32 = 2.0;

pub struct PipePair{
    pub hitboxes: (Rectangle, Rectangle),
    pub gap: f32,
    pub vel: f32,
}

impl PipePair{
    pub fn new() -> Self{
        let top = Rectangle::new((1000.0, 0.0), (WIDTH, 300.0));
        let bottom = Rectangle::new((1000.0, INIT_GAP), (WIDTH, 300.0));

        PipePair{
            hitboxes: (top, bottom),
            gap: INIT_GAP,
            vel: 0.0,
        }
    }

    pub fn update(&mut self){
        if self.hitboxes.0.x() + WIDTH > 0.0 {
            let delta_gap = self.vel + GAP_ACCEL/2.0;
            self.gap -= delta_gap;
            js!{ console.log(@{self.gap}); }
            let penalty = if self.vel < 0.0 {self.vel/3.0} else {0.0};
            let top = self.hitboxes.0.translate((-1.0 * SPEED + penalty, -1.0 * delta_gap/2.0));
            let bottom = top.translate((0.0, self.gap));
            self.hitboxes = (top, bottom);
            if self.gap >= 800.0 || self.gap <= 300.0{
                if self.gap >= 800.0 {self.gap = 800.0}
                else if self.gap <= 300.0 {self.gap = 300.0};
                self.vel = 0.0;
            }else{
                self.vel += GAP_ACCEL;
            }
        }else {
            let y_pos = rand::random::<f32>() * 700.0 - 400.0;

            let top = Rectangle::new((1000.0, y_pos), (WIDTH, 300.0));
            let bottom = Rectangle::new((1000.0, y_pos + INIT_GAP), (WIDTH, 300.0));
            self.hitboxes = (top, bottom);
        }
    }

    pub fn jump(&mut self){
        if self.gap >= 100.0{
            self.vel -= 10.0;
        }
    }
}
