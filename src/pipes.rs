// use stdweb;
// use stdweb::js;

use quicksilver::{
    geom::{Rectangle, Shape},
};

const WIDTH: f32 = 100.0;
const INIT_GAP: f32 = 200.0;
const GAP_ACCEL: f32 = -0.85;
const HURTBOX_SPEED: f32 = 1.5;

pub struct PipePair{
    pub hitboxes: (Rectangle, Rectangle),
    pub hurtboxes: (Rectangle, Rectangle),
    pub center_y: f32,
    pub gap: f32,
    pub vel: f32,
    pub speed: f32,
    pub score: u32,
    pub hurtbox_vel: f32,
    pub max_gap: f32,
}

impl PipePair{
    pub fn new() -> Self{
        let top = Rectangle::new((1000.0, -200.0), (WIDTH, 500.0));
        let bottom = Rectangle::new((1000.0, INIT_GAP), (WIDTH, 300.0));

        let top_h = Rectangle::new((0.0, 0.0), (1000.0, 400.0/2.0));
        let bottom_h = Rectangle::new((0.0, 600.0), (1000.0, 400.0/2.0));

        PipePair{
            hitboxes: (top, bottom),
            hurtboxes: (top_h, bottom_h),
            center_y: 400.0,
            gap: INIT_GAP,
            vel: 0.0,
            speed: 10.0,
            score: 1,
            hurtbox_vel: HURTBOX_SPEED,
            max_gap: 400.0,
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

            self.center_y += self.hurtbox_vel;

            let top_h = Rectangle::new( (0.0, 0.0), (1000.0, self.center_y - self.max_gap/2.0));
            let bottom_h = Rectangle::new( (0.0, self.center_y + self.max_gap/2.0), (1000.0, self.center_y + 500.0) );
            
            if bottom_h.y() >= 800.0 { self.hurtbox_vel *= -1.0 };
            if top_h.y() + top_h.height() <= 0.0 { self.hurtbox_vel *= -1.0 };

            self.hurtboxes = (top_h, bottom_h);
            self.hitboxes = (top, bottom);

        }else {
            let top = Rectangle::new( (1000.0, 0.0), (WIDTH, self.center_y - INIT_GAP));
            let bottom = Rectangle::new( (1000.0, self.center_y + INIT_GAP), (WIDTH, self.center_y) );



            self.gap = INIT_GAP;
            self.hitboxes = (top, bottom);

            self.speed *= 1.0 + (30.0 - self.speed)/300.0; //logistic I think
            self.max_gap *= 0.95;
            if self.gap >= self.max_gap/2.0 {self.gap = self.max_gap/2.0 * 0.95};
            self.score += 1;
        }
    }

    pub fn jump(&mut self){
        self.vel += 10.0;
    }
}
