use graphics::Context;
use opengl_graphics::GlGraphics;
use geom;
use constante::{WINDOW_WIDTH, WINDOW_HEIGHT, RACKET_VELOCITY, RACKET_WIDTH, RACKET_HEIGHT};

use super::{GameObject, render_racket, Direction};

pub struct Enemy {
    pub score: u32,
    pub pos: geom::Position,
    dir: Direction,
}

impl Enemy {
    pub fn new() -> Enemy{
        Enemy {
            score: 0,
            pos: geom::Position::new(WINDOW_WIDTH as f64 - RACKET_WIDTH,
                                     (WINDOW_HEIGHT / 2) as f64 - RACKET_HEIGHT / 2.),
            dir: Direction::STOP,
        }
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }
}

impl GameObject for Enemy {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
    render_racket(&self.pos, ctxt, gl);
    }

    fn update(&mut self, _: f64) {
        match self.dir {
            Direction::UP => {
                if self.pos.y > 0. {
                    self.pos.y -= RACKET_VELOCITY;
                }
            }
            Direction::DOWN => {
                if self.pos.y + RACKET_HEIGHT < WINDOW_HEIGHT as f64 {
                    self.pos.y += RACKET_VELOCITY;
                }
            }
            _ => {}
        }
    }
}