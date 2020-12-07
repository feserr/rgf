extern crate ganger_engine;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let mut window =
        ganger_engine::window::Window::new("Hello triangle", SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();

    window.canvas_.set_draw_color(pixels::Color::RGB(0, 0, 0));
    window.canvas_.clear();

    ganger_engine::draw::draw_hello_triangle(
        &window.canvas_,
        SCREEN_WIDTH as i16,
        SCREEN_HEIGHT as i16,
    );

    window.canvas_.present();

    let mut events = window.sdl_context_.event_pump()?;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                _ => {}
            }
        }
    }

    Ok(())
}
