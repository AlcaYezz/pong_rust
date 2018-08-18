use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;

use geom::*;
use constante::*;

pub mod enemy;
pub mod player;
pub mod ball;

pub enum Direction {
    UP,
    DOWN,
    STOP,
}

pub trait GameObject {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics);

    fn update(&mut self, _: f64) {}
}

pub fn render_racket(pos: &Position, ctxt: &Context, gl: &mut GlGraphics)
{
    use constante::{RACKET_HEIGHT, RACKET_WIDTH};
    rectangle(RACKET_COLOR,
              rectangle::rectangle_by_corners(0., 0., RACKET_WIDTH, RACKET_HEIGHT),
              ctxt.transform.trans(pos.x, pos.y),
              gl);
}

pub fn move_enemy(enemy: &mut enemy::Enemy, ball: &ball::Ball) {
    if ball.pos.y < enemy.pos.y + RACKET_HEIGHT * 0.25 && enemy.pos.y > 0. {
        enemy.set_dir(Direction::UP);
    } else if ball.pos.y > enemy.pos.y + RACKET_HEIGHT * 0.75 && enemy.pos.y + RACKET_HEIGHT < WINDOW_HEIGHT as f64 {
        enemy.set_dir(Direction::DOWN);
    } else {
        enemy.set_dir(Direction::STOP);
    }
}

pub fn collide(ball: &mut ball::Ball, enemy: &mut enemy::Enemy, player: &mut player::Player) {
    if ball.pos.x <= 0. {
        enemy.score += 1;
        ball.reset();
    } else if ball.pos.x >= WINDOW_WIDTH as f64 {
        player.score += 1;
        ball.reset();
    } else if ball.pos.y - BALL_RADIUS <= 0. || ball.pos.y + BALL_RADIUS >= WINDOW_HEIGHT as f64 {
        ball.dir.y *= -1.;
    }

    let start_player = &Position::new(player.pos.x + RACKET_WIDTH,
                                      player.pos.y);

    collide_racket(ball, &enemy.pos, false);
    collide_racket(ball, start_player, true);
}

fn collide_racket(ball: &mut ball::Ball, racket: &Position, is_player: bool) {
    let end = Position::new(racket.x, racket.y + RACKET_HEIGHT);

    if collide_edge(ball, racket, end) {
        let relative = racket.y + RACKET_HEIGHT - ball.pos.y;
        let normalize = relative / RACKET_HEIGHT;
        ball.angle = clean_angle(if is_player {
            ((MAX_ANG_PLAYER - MIN_ANG_PLAYER) * normalize + MIN_ANG_PLAYER)
        } else {
            ((MAX_ANG_ENEMY - MIN_ANG_ENEMY) * normalize + MIN_ANG_ENEMY)
        });
        ball.get_velocity();
    }
}

fn collide_edge(ball: &mut ball::Ball, origin: &Position, end: Position) -> bool {
    let d = Vector::new(end.x - origin.x,
                        end.y - origin.y);
    let f = Vector::new(origin.x - ball.pos.x,
                        origin.y - ball.pos.y);

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
