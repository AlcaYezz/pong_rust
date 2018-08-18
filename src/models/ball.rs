use graphics::{Context, ellipse, Transformed};
use opengl_graphics::GlGraphics;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use geom::{Position, Vector, clean_angle};
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

        let start_player = &Position::new(player.pos.x + RACKET_WIDTH,
                                          player.pos.y);

        self.collide_racket(&enemy.pos, false);
        self.collide_racket(start_player, true);


        self.pos.x += self.vx;
        self.pos.y += self.vy;
    }

    fn collide_racket(&mut self, racket: &Position, is_player: bool) {
        let end = Position::new(racket.x, racket.y + RACKET_HEIGHT);

        if self.collide_edge(racket, end) {
            let relative = racket.y + RACKET_HEIGHT - self.pos.y;
            let normalize = relative / RACKET_HEIGHT;
            self.angle = clean_angle(if is_player {
                ((MAX_ANG_PLAYER - MIN_ANG_PLAYER) * normalize + MIN_ANG_PLAYER)
            } else {
                ((MAX_ANG_ENEMY - MIN_ANG_ENEMY) * normalize + MIN_ANG_ENEMY)
            });
            self.get_velocity();
        }
    }

    fn collide_edge(&mut self, origin: &Position, end: Position) -> bool {
        let d = Vector::new(end.x - origin.x,
                            end.y - origin.y);
        let f = Vector::new(origin.x - self.pos.x,
                            origin.y - self.pos.y);

        let a = d.dot(d);
        let b = f.dot(d) * 2.;
        let c = f.dot(f) - BALL_RADIUS * BALL_RADIUS;

        let mut delta = b * b - 4. * a * c;
        if delta < 0. {
            false
        } else {
            delta = delta.sqrt();
            let x1 = (-b - delta) / (2. * a);
            let x2 = (-b + delta) / (2. * a);

            if (x1 >= 0. && x1 <= 1.) ||
                (x2 >= 0. && x2 <= 1.) {
                true
            } else {
                false
            }
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