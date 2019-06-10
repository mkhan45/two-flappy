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

use stdweb::unstable::TryInto;

struct MainState {
    bird: Player,
    pipes: PipePair,
    alive: bool,
    score_img: Option<Image>,
}

impl State for MainState{
    fn new() -> Result<Self> {

        let state = MainState{
            bird: Player::new(20.0, 200.0),
            pipes: PipePair::new(),
            alive: true,
            score_img: None,
        };

        // let mut text_renderer = Asset::new(Font::load("OpenSans-Regular.ttf"));
        // let font_style =  FontStyle::new(48.0, Color::BLACK);

        // text_renderer.execute(|font| {
        //     let image = font.render("Score: 0", &font_style);
        //     match image{
        //         Ok(img) => state.score_img = Some(img),
        //         Err(_e) => println!("error loading font"),
        //     }

        //     Ok(())
        // }).expect("error font");

        Ok(state)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        js!{
            js_update();
        }
        if self.alive{
            self.bird.update();
            self.pipes.update();


            if (self.bird.hitbox.overlaps(&self.pipes.hitboxes.0) || self.bird.hitbox.overlaps(&self.pipes.hitboxes.1)
                || self.bird.hitbox.x() == self.pipes.hitboxes.0.x() && self.bird.hitbox.y() <= -5.0
                || self.bird.hitbox.y() >= 805.0
                || self.pipes.gap > self.pipes.max_gap/2.0)
                && self.alive{
                    js!{ @(no_return)
                        document.title = @{self.pipes.score} + " - Dead";
                    }
                    self.alive = false;
                }

            if self.pipes.hitboxes.0.x() <= 0.0 && self.alive{

                // let score = format!("Score: {}", self.pipes.score);
                // let font_style =  FontStyle::new(48.0, Color::BLACK);

                js!{ @(no_return)
                    document.title = @{self.pipes.score};
                }
            }
        }

        let socket_jump: bool = js!{
            if(socket_jump)
                return true;
            else
                return false;
        }.try_into().unwrap();

        let socket_pipe: bool = js!{
            if(socket_pipe)
                return true;
            else
                return false;
        }.try_into().unwrap();

        let socket_reset: bool = js!{
            if(socket_reset)
                return true;
            else
                return false;
        }.try_into().unwrap();

        if socket_jump{
            self.bird.jump();
            js!{
                socket_jump = false;
            }
        }

        if socket_pipe{
            self.pipes.jump();
            js!{
                socket_pipe = false;
            }
        }

        if socket_reset{
            self.bird = Player::new(20.0, 200.0);
            self.pipes = PipePair::new();
            self.alive = true;
            self.score_img = None;
            js!{
                socket_reset = false;
            }
        }


        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let hurtboxes_color = Background::Col(Color::from_rgba(255, 0, 0, 0.25));

        window.draw(&self.pipes.hitboxes.0, Background::Col(Color::GREEN));
        window.draw(&self.pipes.hitboxes.1, Background::Col(Color::BLUE));
        window.draw(&self.pipes.hurtboxes.0, hurtboxes_color);
        window.draw(&self.pipes.hurtboxes.1, hurtboxes_color);
        window.draw(&self.bird.hitbox, Background::Col(Color::BLACK));

        match &self.score_img{
            Some(img) => window.draw(&img.area(), Img(&img)),
            None => {},
        };

        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event{
            // Event::MouseButton(btn, state) => {
            //     if btn == &MouseButton::Left && state == &ButtonState::Pressed && self.alive{
            //         self.bird.jump();
            //     }
            // },

            Event::Key(btn, state) => {
                if state == &ButtonState::Pressed{
                    match btn{
                        Key::W => self.pipes.jump(),
                        Key::Up => self.bird.jump(),
                        Key::S => {js!{socket_jump_queue = true}; ()},
                        Key::F => {js!{socket_pipe_queue = true}; ()},
                        Key::R => {
                            self.bird = Player::new(20.0, 200.0);
                            self.pipes = PipePair::new();
                            self.alive = true;
                            self.score_img = None;
                        },
                        Key::N => {
                            js!{
                                socket_reset_queue = true;
                            }
                        },

                        _ => {},
                    };
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

#[js_export]
pub fn test_js() -> u32{
    js!{
        console.log("worked");
    };
    5
}
