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
use opengl_graphics::{OpenGL};

use app::App;
use constante::{WINDOW_HEIGHT, WINDOW_WIDTH};


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [WINDOW_WIDTH, WINDOW_HEIGHT]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);

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

        if let Some(_) = e.update_args() {
            app.update();
        }
    }
}