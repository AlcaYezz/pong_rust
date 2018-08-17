use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;
use piston::window::Size;

use geom::Position;

pub mod enemy;
pub mod player;
pub mod ball;

const RACKET_COLOR: [f32; 4] = [0.25, 0.25, 1., 1.0];

pub trait GameObject {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics);

    fn update(&mut self, _: f64, _: Size) {}
}

pub fn render_racket(pos: &Position, ctxt: &Context, gl: &mut GlGraphics)
{
    use constante::{RACKET_HEIGHT, RACKET_WIDTH};
    rectangle(RACKET_COLOR,
              rectangle::rectangle_by_corners(0., 0., RACKET_WIDTH, RACKET_HEIGHT),
              ctxt.transform.trans(pos.x, pos.y),
              gl);
}
