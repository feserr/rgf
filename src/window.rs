extern crate sdl2;

pub struct Window {
  pub sdl_context_: sdl2::Sdl,
  pub video_subsystem_: sdl2::VideoSubsystem,
  pub canvas_: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
  pub fn new(name: &str, width: u32, height: u32) -> Result<Window, String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
      .window(name, width, height)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    Ok(Window {
      sdl_context_: sdl_context,
      video_subsystem_: video_subsys,
      canvas_: canvas,
    })
  }
}
