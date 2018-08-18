use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston::input::*;

use models::{GameObject, collide, move_enemy};
use models::enemy::Enemy;
use models::player::Player;
use models::ball::Ball;

pub struct App<'a> {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub glyphs: GlyphCache<'a>,
    pub ball: Ball,
    pub enemy: Enemy,
    pub player: Player,
}

impl<'a> App<'a> {
    pub fn new(opengl: OpenGL, glyphs: GlyphCache<'a>) -> App<'a> {
        App {
            gl: GlGraphics::new(opengl),
            glyphs,
            ball: Ball::new(),
            enemy: Enemy::new(),
            player: Player::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.75, 1.0, 0.5, 1.0];
        const BLACK:   [f32; 4] = [0., 0., 0., 1.0];

        let enemy = &self.enemy;
        let player = &self.player;
        let ball = &self.ball;

        let glyph_cache = &mut self.glyphs;

        self.gl.draw(args.viewport(), |c, gl| {

            // Draw the field.
            clear(GREEN, gl);

            line(BLACK, 1., [args.width as f64 / 2., 0., args.width as f64 / 2., args.height as f64], c.transform, gl);
            text::Text::new_color(BLACK, 32)
                .draw(
                    format!("{}", player.score).as_str(),
                    glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(args.width as f64 * 0.25, args.height as f64 * 0.5),
                    gl
                ).unwrap();
            text::Text::new_color(BLACK, 32)
                .draw(
                    format!("{}", enemy.score).as_str(),
                    glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(args.width as f64 * 0.75, args.height as f64 * 0.5),
                    gl
                ).unwrap();

            //Render element
            enemy.render(&c, gl);
            player.render(&c, gl);
            ball.render(&c, gl);
        });
    }

    pub fn update(&mut self, dt: f64) {
        collide(&mut self.ball, &mut self.enemy, &mut self.player);
        move_enemy(&mut self.enemy, &self.ball);
        self.ball.update(dt);
        self.enemy.update(dt);
        self.player.update(dt);
    }

    pub fn input(&mut self, key: Key, is_pressed: bool) {
        if is_pressed {
            if key == Key::Up || key == Key::Z {
                self.player.move_up();
            } else if key == Key::Down || key == Key::S {
                self.player.move_down();
            }
        } else {
            self.player.move_stop();
        }
    }
}