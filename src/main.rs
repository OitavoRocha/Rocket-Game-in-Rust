// Neste jogo, as entradas são o peso de uma nave, quantidade de combustível da nave e a gravidade do planeta onde a nave aterriza. (FEITO)
// Implementar o recebimento de combustível. (FEITO)
// Aumentar o uso de combustível conforme aumenta a velocidade, e vice-versa. (FEITO)
// Criar uma função para a velocidade baseado na gravidade. (FEITO +/-)
// O objetivo é aterrizar com segurança no alvo. (FEITO)
// Com as setas "para cima" e "para baixo" o jogador pode aumentar ou diminuir a quantidade de combustível injetada, aumentando ou diminuindo a potência do motor. (FEITO)
// Caso a potência alcançada seja maior que a gravidade, a nave sobe. (FEITO +/-)
// Ao acabar o combustível, a nave cai e o jogador perde. (FEITO)
// Caso a nave chegue ao chão muito acelerada, a nave também é destruída e o jogador perde. (FEITO)
// As setinhas para os lados movem a nave para os lados de forma a se enquadrar no alvo. (FEITO)
// Fazer a nave nascer em uma área específica aleatoriamente. (FEITO)
// Fazer uma função pra terminar o jogo. (FEITO)

use ggez::*;
use ggez::graphics::Text;
use ggez::{Context, GameResult};
use ggez::event;
extern crate nalgebra as na;
use ggez::input::keyboard::KeyCode;
use rand::{thread_rng, Rng};

const ROCKET_WIDTH: f32 = 50.0;
const ROCKET_HEIGHT: f32 = 100.0;
const PLAT_WIDTH: f32 = 200.0;
const PLAT_HEIGHT: f32 = 20.0;
const PLAT_X: f32 = 320.00;
const PLAT_Y: f32 = 580.00;
const MIN_HEIGHT: f32 = 500.0;
const MAX_WIDTH: f32 = 750.00;
const MAX_FUEL: f32 = 1000.0; // Fuel goes up to 1 ton, if its at maximum it's 100%

struct State {
    rocket_x: f32,
    rocket_y: f32,
    speed: f32,
    fuel: f32,
    injected_fuel: f32,
    max_speed: f32,
    min_speed: f32,
}

impl State {
    pub fn new( x: f32, fuel: f32, grav: f32 ) -> Self {
        State{
            rocket_x: x,
            rocket_y: 10.0,
            speed: grav,
            fuel: fuel,
            injected_fuel: 0.05,
            max_speed: grav,
            min_speed: grav * 0.4,
        }
    }
}

impl event::EventHandler<> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let k_ctx = &_ctx.keyboard;

        // Variable to determine if the rocket had or not touched the platform
        let intersects_platform = self.rocket_y > (PLAT_Y - ROCKET_HEIGHT) && self.rocket_x > (PLAT_X - ROCKET_WIDTH + 1.0)
        && self.rocket_x < (PLAT_X + PLAT_WIDTH - 1.0 ); 


        // Conditions to win the game
        if self.fuel >= 0.0 {
            if intersects_platform {
                if self.speed <= self.min_speed {
                    end_game(1)
                } else {
                    end_game(-1);
                }
            } else if self.rocket_y >= MIN_HEIGHT {
                end_game(-1);
            } else {
                self.rocket_y += self.speed;
            }
        } else {
            end_game(-1);
        }


        if self.speed < self.max_speed {
            self.fuel -= self.injected_fuel;
        }

        //
        // PRESSED BUTTONS TREATMENT
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
                self.speed -= 0.5;
                self.injected_fuel += 0.05;
        } else if k_ctx.is_key_just_pressed(KeyCode::Down) {
            if self.speed <= self.max_speed - self.injected_fuel {
                self.speed += 0.5;
                self.injected_fuel -= 0.05;
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

        let text = Text::new(format!("Fuel: {:.0}%\nSpeed: {}", self.fuel, self.speed));

        canvas.draw(&rocket, graphics::DrawParam::default());
        canvas.draw(&platform, graphics::DrawParam::default());
        canvas.draw(&text, graphics::DrawParam::new().dest([5.0, 5.0]));
        canvas.finish(_ctx)?;
        
        Ok(())
    }
}

fn end_game( result: i32 ) -> !{
    match result {
        0 => println!("Invalid Arguments./n"),
        1 => println!("You Won!!"),
        _ => println!("You Lost!! :c"),
    }
    std::process::exit(0)
}

fn main() -> GameResult {
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        end_game(0);
    }

    let cb = ggez::ContextBuilder::new("Rocket_game", "Otavio Rocha");
    let (ctx, event_loop) = cb.build().unwrap();
    ctx.gfx.set_window_title("Rocket Game");
    let mut rng = thread_rng();

    let fuel: f32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            end_game(0);
        }
    };

    let gravity: f32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            end_game(0);
        }
    };

    let weight: f32 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            end_game(0);
        }
    };

    if weight < fuel {
        end_game(0);
    }

    let state = State::new( (rng.gen_range(0.0..1.0) * 200.0) + 275.0,
                                    (100.0 * fuel) / MAX_FUEL , gravity);
    event::run(ctx, event_loop, state);
}