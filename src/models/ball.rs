use graphics::{Context, ellipse, Transformed};
use opengl_graphics::GlGraphics;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use geom::{Position, clean_angle};
use constante::*;
use super::GameObject;
use super::enemy::Enemy;
use super::player::Player;

pub struct Ball {
    pub pos: Position,
    vx: f64,
    vy: f64,
    angle: f64,
}

impl Ball {
    pub fn new() -> Ball{
        let mut ball = Ball {
            pos: Position::new(WINDOW_WIDTH as f64 / 2.,
                               WINDOW_HEIGHT as f64 / 2.),
            vx: 0.,
            vy: 0.,
            angle: 0.,
        };
        ball.reset();
        ball
    }

    fn reset(&mut self) {
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
    }

    pub fn get_velocity(&mut self) {
        self.vx = BALL_VELOCITY * self.angle.cos();
        self.vy = BALL_VELOCITY * self.angle.sin();
    }

    pub fn update(&mut self, enemy: &mut Enemy, player: &mut Player) {
        if self.pos.x - BALL_RADIUS <= 0. {
            enemy.score += 1;
            self.reset();
        } else if self.pos.x + BALL_RADIUS >= WINDOW_WIDTH as f64{
            player.score += 1;
            self.reset();
        } else if self.pos.y - BALL_RADIUS <= 0. || self.pos.y + BALL_RADIUS >= WINDOW_HEIGHT as f64{
            self.vy *= -1.;
        }
        let mut min = (clean_angle(self.angle - PI / 2.) * 1000.) as u32;
        let mut max = (clean_angle(self.angle + PI / 2.) * 1000.) as u32;
        if min > max {
            let tmp = min;
            min = max;
            max = tmp;
        }

        //for i in 0..6283 {
        for ti in min..max {
            let angle = i as f64 / 1000.;
            let pos = Position::new(self.pos.x + BALL_RADIUS * angle.cos(),
                                             self.pos.y + BALL_RADIUS * angle.sin());

            self.colide_racket(&enemy.pos, &pos, false);
            self.colide_racket(&player.pos, &pos, true);
        }
        self.pos.x += self.vx;
        self.pos.y += self.vy;
    }

    fn colide_racket(&mut self, racket: &Position, pos: &Position, is_player: bool) {
        if racket.x <= pos.x && racket.x + RACKET_WIDTH >= pos.x &&
            racket.y <= pos.y && racket.y + RACKET_HEIGHT >= pos.y {
            let relative = racket.y + RACKET_HEIGHT - pos.y;
            let normalize = relative / RACKET_HEIGHT;
            self.angle = clean_angle(if is_player {
                ((MAX_ANG_PLAYER - MIN_ANG_PLAYER) * normalize + MIN_ANG_PLAYER)
            } else {
                ((MAX_ANG_ENEMY - MIN_ANG_ENEMY) * normalize + MIN_ANG_ENEMY)
            });
            self.get_velocity();
        }
    }
}

impl GameObject for Ball {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        ellipse(BALL_COLOR,
                ellipse::circle(0.0, 0.0, BALL_RADIUS),
                ctxt.transform.trans(self.pos.x, self.pos.y),
                gl);
    }
}