extern crate quicksilver;
use quicksilver::{
    Result,
    geom::{Rectangle, Transform, Vector, Shape},
    graphics::{Background, Background::Col, Color, Drawable},
    lifecycle::{Settings, State, Window, run, Event},
    input::{MouseButton, Key, ButtonState}
};

mod Bird;
use Bird::Player;

struct MainState {
    score: i32,
    bird: Player,
}

impl State for MainState{
    fn new() -> Result<Self> {
        Ok(MainState{
            score: 0,
            bird: Player::new(10.0, 200.0),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.bird.update();
        
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        window.draw(&self.bird.hitbox, Background::Col(Color::BLACK));

        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event{
            Event::MouseButton(btn, state) => {
                if btn == &MouseButton::Left && state == &ButtonState::Pressed{
                    self.bird.jump();
                }
            },

            _ => {},
        };
        Ok(())
    }
}

pub fn main(){
    run::<MainState>("N-body Gravity Sim", Vector::new(1000, 800), Settings {
        draw_rate: 1.0,  //draw as fast as possible basically
        update_rate: 1000. / 30., 
        vsync: true,
        ..Settings::default()
    });

    println!("test");
}
