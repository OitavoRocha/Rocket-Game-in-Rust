// Neste jogo, as entradas são o peso de uma nave, quantidade de combustível da nave e a gravidade do planeta onde a nave aterriza. 
// O objetivo é aterrizar com segurança no alvo. (FEITO)
// Com as setas "para cima" e "para baixo" o jogador pode aumentar ou diminuir a quantidade de combustível injetada, aumentando ou diminuindo a potência do motor. (FEITO)
// Caso a potência alcançada seja maior que a gravidade, a nave sobe. 
// Ao acabar o combustível, a nave cai e o jogador perde. (FEITO)
// Caso a nave chegue ao chão muito acelerada, a nave também é destruída e o jogador perde. (FEITO)
// As setinhas para os lados movem a nave para os lados de forma a se enquadrar no alvo. (FEITO)
use ggez::*;
use ggez::graphics::Text;
use ggez::{Context, GameResult};
use ggez::event;
extern crate nalgebra as na;
use ggez::input::keyboard::KeyCode;

const ROCKET_WIDTH: f32 = 50.0;
const ROCKET_HEIGHT: f32 = 100.0;
const PLAT_WIDTH: f32 = 400.0;
const PLAT_HEIGHT: f32 = 20.0;
const PLAT_X: f32 = 200.00;
const PLAT_Y: f32 = 580.00;
const MIN_HEIGHT: f32 = 500.0;
const MAX_WIDTH: f32 = 750.00;
const MAX_SPEED: f32 = 3.5;

static mut CICLE_COUNTER: i32 = 0;

struct State {
    rocket_x: f32,
    rocket_y: f32,
    speed: f32,
    fuel: f32,
}


impl State {
    pub fn new() -> Self {
        State{
            rocket_x: 375.0,
            rocket_y: 10.0,
            speed: MAX_SPEED,
            fuel: 100.0,
        }
    }
}

impl event::EventHandler<> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let intersects_platform = self.rocket_y > (PLAT_Y - ROCKET_HEIGHT) && self.rocket_x > (PLAT_X - ROCKET_WIDTH + 1.0)
        && self.rocket_x < (PLAT_X + PLAT_WIDTH - 1.0 ); // Variable to determine if the rocket had or not touched the platform
        let k_ctx = &_ctx.keyboard;

        // Conditions to win the game
        if self.fuel >= 5.0 {
            if intersects_platform {
                if self.speed <= 2.0 {
                    println!("You Won!");
                } else {
                    println!("You Lost!");
                }
            } else if self.rocket_y >= MIN_HEIGHT {
                println!("You Lost!");
            } else {
                self.rocket_y += self.speed;
            }
            unsafe { CICLE_COUNTER += 1; }
        } else {
            println!("You Lost!");
        }

        unsafe {
            if CICLE_COUNTER == 50 {
                self.fuel -= 10.0;
                CICLE_COUNTER = 0;
            }
        }
        
        if k_ctx.is_key_pressed(KeyCode::Left){
            if self.rocket_x >= 5.0 {
                self.rocket_x -= 5.0;
            }
        } else if k_ctx.is_key_pressed(KeyCode::Right) {
            if self.rocket_x <= MAX_WIDTH - 5.0 {
                self.rocket_x += 5.0;
            }
        }
        
        if k_ctx.is_key_just_pressed(KeyCode::Up) {
            if self.speed > 0.5 {
                self.speed -= 0.5;
            }
        } else if k_ctx.is_key_just_pressed(KeyCode::Down) {
            if self.speed <= MAX_SPEED - 0.5 {
                self.speed += 0.5;
            }
        }


        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::BLACK);
        

        let rocket = graphics::Mesh::new_rectangle(
            _ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect { x: self.rocket_x, y: self.rocket_y, w: ROCKET_WIDTH, h: ROCKET_HEIGHT }, 
            graphics::Color::WHITE,
        )?;

        let platform = graphics::Mesh::new_rectangle(
            _ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect { x: PLAT_X, y: PLAT_Y, w: PLAT_WIDTH, h: PLAT_HEIGHT},
            graphics::Color::BLUE,
        )?;

        let text = Text::new(format!("Fuel: {}%\nSpeed: {}", self.fuel, self.speed));

        canvas.draw(&rocket, graphics::DrawParam::default());
        canvas.draw(&platform, graphics::DrawParam::default());
        canvas.draw(&text, graphics::DrawParam::new().dest([5.0, 5.0]));
        canvas.finish(_ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Rocket_game", "Otavio Rocha");
    let (ctx, event_loop) = cb.build().unwrap();
    ctx.gfx.set_window_title("Rocket Game");


    let state = State::new();
    event::run(ctx, event_loop, state);
}