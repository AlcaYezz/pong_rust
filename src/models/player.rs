use graphics::Context;
use opengl_graphics::GlGraphics;
use geom;
use constante::{WINDOW_HEIGHT, RACKET_VELOCITY, RACKET_HEIGHT};
use super::{GameObject, render_racket};

enum Direction {
    UP,
    DOWN,
    STOP,
}

pub struct Player {
    pub score: u32,
    pub pos: geom::Position,
    dir: Direction,
}

impl Player {
    pub fn new() -> Player {
        Player {
            score: 0,
            pos: geom::Position::new(0.,
                                     (RACKET_HEIGHT as f64 / 2.) - RACKET_HEIGHT as f64 / 2.),
            dir: Direction::STOP,
        }
    }

    pub fn move_up(&mut self) {
        self.dir = Direction::UP;
    }

    pub fn move_down(&mut self) {
        self.dir = Direction::DOWN;
    }

    pub fn move_stop(&mut self) {
        self.dir = Direction::STOP;
    }

    pub fn update(&mut self) {
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
            _ =>  {}
        }
    }
}


impl GameObject for Player {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        render_racket(&self.pos, ctxt, gl);
    }
}