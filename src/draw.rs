extern crate sdl2;

use sdl2::gfx::primitives::DrawRenderer;

pub fn draw_hello_triangle(
  canvas: &sdl2::render::Canvas<sdl2::video::Window>,
  width: i16,
  height: i16,
) -> Result<(), String> {
  canvas.filled_trigon(
    width / 2 - width / 4,
    height / 2 + height / 4,
    width / 2,
    height / 2 - height / 4,
    width / 2 + width / 4,
    height / 2 + height / 4,
    0xFFFFFFFFu32,
  )?;

  Ok(())
}
