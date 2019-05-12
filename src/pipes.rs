use stdweb;
use stdweb::js;

use quicksilver::{
    geom::{Rectangle, Shape},
};

use rand;

const WIDTH: f32 = 100.0;
const INIT_GAP: f32 = 200.0;
const GAP_ACCEL: f32 = -0.85;

pub struct PipePair{
    pub hitboxes: (Rectangle, Rectangle),
    pub center_y: f32,
    pub gap: f32,
    pub vel: f32,
    pub speed: f32,
    pub score: u32,
}

impl PipePair{
    pub fn new() -> Self{
        let top = Rectangle::new((1000.0, 0.0), (WIDTH, 300.0));
        let bottom = Rectangle::new((1000.0, INIT_GAP), (WIDTH, 300.0));

        PipePair{
            hitboxes: (top, bottom),
            center_y: 400.0,
            gap: INIT_GAP,
            vel: 0.0,
            speed: 10.0,
            score: 1,
        }
    }

    pub fn update(&mut self){
        if self.hitboxes.0.x() + WIDTH > 0.0 {

            let delta_gap = self.vel;


            if self.gap >= 400.0{
                self.gap = 400.0;
                if delta_gap <= 0.0{
                    self.gap += delta_gap;
                }
                self.vel = 0.0;
                self.vel += GAP_ACCEL;
            }else if self.gap <= 0.0{
                self.gap = 0.0;
                if delta_gap >= 0.0{
                    self.gap += delta_gap;
                }
                if self.vel <= 0.0{
                    self.vel = 0.0;
                }
            }else{
                self.gap += delta_gap;
                self.vel += GAP_ACCEL;
            }

            // js!{ console.log(@{self.gap} + " " + @{delta_gap} + " " + @{self.vel}) }

            let top = Rectangle::new((self.hitboxes.0.x() - self.speed, 0.0), (WIDTH, self.center_y - self.gap));
            let bottom = Rectangle::new( (self.hitboxes.0.x() - self.speed, self.center_y + self.gap), (WIDTH, self.center_y + 500.0) );

            self.hitboxes = (top, bottom);

        }else {
            self.center_y = rand::random::<f32>() * 400.0 + 200.0;
            let top = Rectangle::new( (1000.0, 0.0), (WIDTH, self.center_y - INIT_GAP));
            let bottom = Rectangle::new( (1000.0, self.center_y + INIT_GAP), (WIDTH, self.center_y) );

            self.gap = INIT_GAP;
            self.hitboxes = (top, bottom);

            self.speed *= 1.0 + (30.0 - self.speed)/300.0;
            self.score += 1;
        }
    }

    pub fn jump(&mut self){
        self.vel += 5.0;
    }
}
