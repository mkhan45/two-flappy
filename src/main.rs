extern crate quicksilver;
use quicksilver::{
    Future, Result,
    combinators::result,
    geom::{Rectangle, Transform, Vector, Shape},
    graphics::{Background, Background::Col, Color, Drawable, Font, FontStyle, Background::Img, Image},
    lifecycle::{Settings, State, Window, run, Event, Asset},
    input::{MouseButton, Key, ButtonState}
};

mod Bird;
use Bird::Player;

mod Pipes;
use Pipes::PipePair;

struct MainState {
    score: i32,
    bird: Player,
    pipes: PipePair,
    alive: bool,
    score_img: Option<Image>,
}


impl State for MainState{
    fn new() -> Result<Self> {

        let mut state = MainState{
            score: 0,
            bird: Player::new(10.0, 200.0),
            pipes: PipePair::new(),
            alive: true,
            score_img: None,
        };

        let mut text_renderer = Asset::new(Font::load("OpenSans-Regular.ttf"));
        let font_style =  FontStyle::new(48.0, Color::BLACK);

        text_renderer.execute(|font| {
            let image = font.render("Score: 0", &font_style);
            match image{
                Ok(img) => state.score_img = Some(img),
                Err(_e) => println!("error loading font"),
            }

            Ok(())
        }).expect("error font");

        Ok(state)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.bird.update();
        self.pipes.update();

        if self.bird.hitbox.overlaps(&self.pipes.hitboxes.0) || self.bird.hitbox.overlaps(&self.pipes.hitboxes.1){
            println!("dead");
            self.alive = false;
        }

        if self.bird.hitbox.x() == self.pipes.hitboxes.0.x(){
            self.score += 1;

            let score = format!("Score: {}", self.score);
            let font_style =  FontStyle::new(48.0, Color::BLACK);

            let mut text_renderer = Asset::new(Font::load("OpenSans-Regular.ttf"));

            text_renderer.execute(|font| {
                let image = font.render(&score, &font_style);
                match image{
                    Ok(img) => self.score_img = Some(img),
                    Err(_e) => println!("error loading font"),
                }

                Ok(())
            }).expect("error font");
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        window.draw(&self.bird.hitbox, Background::Col(Color::BLACK));
        window.draw(&self.pipes.hitboxes.0, Background::Col(Color::BLACK));
        window.draw(&self.pipes.hitboxes.1, Background::Col(Color::BLACK));

        match &self.score_img{
            Some(img) => window.draw(&img.area(), Img(&img)),
            None => {},
        };

        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event{
            Event::MouseButton(btn, state) => {
                if btn == &MouseButton::Left && state == &ButtonState::Pressed && self.alive{
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
