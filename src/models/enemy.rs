use graphics::Context;
use opengl_graphics::GlGraphics;
use geom;
use constante::{WINDOW_WIDTH, WINDOW_HEIGHT, RACKET_VELOCITY, RACKET_WIDTH, RACKET_HEIGHT};

use super::{GameObject, render_racket};
use super::ball::Ball;

pub struct Enemy {
    pub score: u32,
    pub pos: geom::Position,
}

impl Enemy {
    pub fn new() -> Enemy{
        Enemy {
            score: 0,
            pos: geom::Position::new(WINDOW_WIDTH as f64 - RACKET_WIDTH,
                                     (WINDOW_HEIGHT / 2) as f64 - RACKET_HEIGHT / 2.),
        }
    }

    pub fn update(&mut self, ball: &Ball) {
        if ball.pos.y < self.pos.y + RACKET_HEIGHT * 0.25 && self.pos.y > 0. {
            self.pos.y -= RACKET_VELOCITY;
        } else if ball.pos.y > self.pos.y + RACKET_HEIGHT * 0.75 && self.pos.y + RACKET_HEIGHT < WINDOW_HEIGHT as f64 {
            self.pos.y += RACKET_VELOCITY;
        }
    }
}

impl GameObject for Enemy {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
    render_racket(&self.pos, ctxt, gl);
    }
}