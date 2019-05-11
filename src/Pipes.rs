use quicksilver::{
    geom::{Rectangle, Vector, Shape},
};

const SPEED: f32 = 5.0;

pub struct PipePair{
    pub hitboxes: (Rectangle, Rectangle),
}

impl PipePair{
    pub fn new() -> Self{
        let top = Rectangle::new(Vector::new(1000.0, 0.0), Vector::new(50.0, 300.0));
        let bottom = Rectangle::new(Vector::new(1000.0, 400.0), Vector::new(50.0, 300.0));

        PipePair{
            hitboxes: (top, bottom),
        }
    }

    pub fn update(&mut self){

        if self.hitboxes.0.x() + 25.0 > 0.0 {
            let top = self.hitboxes.0.translate(Vector::new(-1.0 * SPEED, 0.0));
            let bottom = self.hitboxes.1.translate(Vector::new(-1.0 * SPEED, 0.0));
            self.hitboxes = (top, bottom);
        }else {
            let top = Rectangle::new(Vector::new(1000.0, 0.0), Vector::new(50.0, 300.0));
            let bottom = Rectangle::new(Vector::new(1000.0, 400.0), Vector::new(50.0, 300.0));
            self.hitboxes = (top, bottom);
        }

    }
}
