use graphics::{Context, ellipse, Transformed};
use opengl_graphics::GlGraphics;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use geom::{Position, Vector};
use constante::*;
use super::GameObject;

pub struct Ball {
    pub pos: Position,
    pub dir: Vector,
    pub angle: f64,
    pub dt: f64,
}

impl Ball {
    pub fn new() -> Ball{
        let mut ball = Ball {
            pos: Position::new(WINDOW_WIDTH as f64 / 2.,
                               WINDOW_HEIGHT as f64 / 2.),
            dir: Vector::new(0., 0.),
            angle: 0.,
            dt: 0.,
        };
        ball.reset();
        ball
    }

    pub fn reset(&mut self) {
        self.pos.x = WINDOW_WIDTH as f64 / 2.;
        self.pos.y = WINDOW_HEIGHT as f64 / 2.;
        let mut rng = thread_rng();
        let tmp = rng.gen_range(2496, 3771);
        let direction = rng.gen_range(0, 2);
        self.angle = if direction == 0 {
            tmp as f64 / 1000.
        } else {
            (tmp as f64 / 1000. + PI) % (2. * PI)
        };
        self.get_velocity();
        self.dt = 0.8;
    }

    pub fn get_velocity(&mut self) {
        self.dir.x = BALL_VELOCITY * self.angle.cos();
        self.dir.y = BALL_VELOCITY * self.angle.sin();
    }
}

impl GameObject for Ball {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        ellipse(BALL_COLOR,
                ellipse::circle(0.0, 0.0, BALL_RADIUS),
                ctxt.transform.trans(self.pos.x, self.pos.y),
                gl);
    }

    fn update(&mut self, dt: f64) {
        if self.dt <= 0. {
            self.pos.x += self.dir.x;
            self.pos.y += self.dir.y;
        } else {
            self.dt -= dt;
        }
    }
}