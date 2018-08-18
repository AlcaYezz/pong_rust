extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;

mod app;
mod models;
mod geom;
mod constante;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, GlyphCache, TextureSettings};

use app::App;
use constante::{WINDOW_HEIGHT, WINDOW_WIDTH};


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [WINDOW_WIDTH, WINDOW_HEIGHT]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    use find_folder;
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");

    let glyph_cache = GlyphCache::new(
        font,
        (),
        TextureSettings::new()
    ).expect("Unable to load font");

    let mut app = App::new(opengl, glyph_cache);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(Button::Keyboard(k)) = e.press_args() {
            app.input(k, true);
        }

        if let Some(Button::Keyboard(k)) = e.release_args() {
            app.input(k, false);
        }

        if let Some(u) = e.update_args() {
            app.update(u.dt);
        }
    }
}