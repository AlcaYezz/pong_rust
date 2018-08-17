
use models::GameObject;
use models::enemy::Enemy;
use models::player::Player;
use models::ball::Ball;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use piston::input::*;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    //pub glyphs: GlyphCache<'a>,
    pub ball: Ball,
    pub enemy: Enemy,
    pub player: Player,
}

impl App {
    pub fn new(opengl: OpenGL) -> App {
        App {
            gl: GlGraphics::new(opengl),
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

        use find_folder;
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font = assets.join("FiraSans-Regular.ttf");

        let mut glyph_cache = GlyphCache::new(
            font,
            (),
            TextureSettings::new()
        ).expect("Unable to load font");

        self.gl.draw(args.viewport(), |c, gl| {

            // Draw the field.
            clear(GREEN, gl);

            line(BLACK, 1., [args.width as f64 / 2., 0., args.width as f64 / 2., args.height as f64], c.transform, gl);
            text::Text::new_color(BLACK, 32)
                .draw(
                    format!("{}", player.score).as_str(),
                    &mut glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(args.width as f64 * 0.25, args.height as f64 * 0.5),
                    gl
                ).unwrap();
            text::Text::new_color(BLACK, 32)
                .draw(
                    format!("{}", enemy.score).as_str(),
                    &mut glyph_cache,
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

    pub fn update(&mut self) {
        self.ball.update(&mut self.enemy, &mut self.player);
        self.enemy.update(&self.ball);
        self.player.update();
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