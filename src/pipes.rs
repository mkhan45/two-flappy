use quicksilver::{
    geom::{Rectangle, Shape},
};

const SPEED: f32 = 10.0;
const WIDTH: f32 = 100.0;

pub struct PipePair{
    pub hitboxes: (Rectangle, Rectangle),
}

impl PipePair{
    pub fn new() -> Self{
        let top = Rectangle::new((1000.0, 0.0), (WIDTH, 300.0));
        let bottom = Rectangle::new((1000.0, 500.0), (WIDTH, 300.0));

        PipePair{
            hitboxes: (top, bottom),
        }
    }

    pub fn update(&mut self){

        if self.hitboxes.0.x() + WIDTH > 0.0 {
            let top = self.hitboxes.0.translate((-1.0 * SPEED, 0.0));
            let bottom = self.hitboxes.1.translate((-1.0 * SPEED, 0.0));
            self.hitboxes = (top, bottom);
        }else {
            let top = Rectangle::new((1000.0, 0.0), (WIDTH, 300.0));
            let bottom = Rectangle::new((1000.0, 500.0), (WIDTH, 300.0));
            self.hitboxes = (top, bottom);
        }

    }
}
