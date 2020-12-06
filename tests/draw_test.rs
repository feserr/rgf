extern crate ganger_engine;
extern crate sdl2;

#[cfg(test)]
mod tests {
    const SCREEN_WIDTH: u32 = 800;
    const SCREEN_HEIGHT: u32 = 600;

    #[test]
    fn draw_triangle() -> Result<(), String> {
        let canvas = ganger_engine::window::Window::new("test", SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap()
            .canvas_;

        ganger_engine::draw::draw_hello_triangle(
            &canvas,
            SCREEN_WIDTH as i16,
            SCREEN_HEIGHT as i16,
        )?;

        Ok(())
    }
}
