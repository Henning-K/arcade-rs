extern crate sdl2;

mod phi;
mod views;

use sdl2::pixels::Color;
use phi::Events;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().expect("SDL2 context init failed.");
    let video = sdl_context.video().expect("SDL2 video context init failed.");

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
                      .position_centered()
                      .opengl()
                      .build()
                      .expect("Creation of window failed.");

    let mut renderer = window.renderer().accelerated().build().expect("Init of renderer failed.");

    // Prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.now.quit || events.now.key_escape == Some(true) {
            break;
        }

        // Render a fully black window.
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
