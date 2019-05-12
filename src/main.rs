extern crate quicksilver;
use quicksilver::{
    Result,
    geom::{Vector, Shape},
    graphics::{Background, Color, Drawable, Font, FontStyle, Background::Img, Image},
    lifecycle::{Settings, State, Window, run, Event, Asset},
    input::{MouseButton, Key, ButtonState}
};

#[macro_use]
extern crate stdweb;
use stdweb::js;

mod bird;
use bird::Player;

mod pipes;
use pipes::PipePair;

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

        if self.bird.hitbox.overlaps(&self.pipes.hitboxes.0) || self.bird.hitbox.overlaps(&self.pipes.hitboxes.1)
            || self.bird.hitbox.x() == self.pipes.hitboxes.0.x() && (self.bird.hitbox.y() <= -5.0){
                js!{ @(no_return)
                    document.title = @{self.pipes.score} + " - Dead";
                }
                self.alive = false;
            }

        if self.pipes.hitboxes.0.x() <= 0.0 && self.alive{

            let score = format!("Score: {}", self.pipes.score);
            let font_style =  FontStyle::new(48.0, Color::BLACK);

            js!{ @(no_return)
                document.title = @{self.pipes.score};
            }


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

        if window.keyboard()[Key::W].is_down(){
            self.pipes.jump();
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let hurtboxes_color = Background::Col(Color::from_rgba(255, 0, 0, 0.25));

        window.draw(&self.bird.hitbox, Background::Col(Color::BLACK));
        window.draw(&self.pipes.hitboxes.0, Background::Col(Color::BLACK));
        window.draw(&self.pipes.hitboxes.1, Background::Col(Color::BLACK));
        window.draw(&self.pipes.hurtboxes.0, hurtboxes_color);
        window.draw(&self.pipes.hurtboxes.1, hurtboxes_color);

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
    run::<MainState>("Two-Flappy", Vector::new(1000, 800), Settings {
        draw_rate: 1.0,  //draw as fast as possible basically
        update_rate: 1000. / 30., 
        vsync: true,
        ..Settings::default()
    });
}
