extern crate sdl2;
extern crate time;

mod phi;
mod views;

fn main() {
    ::phi::spawn("ArcadeRS Shooter", |_| Box::new(::views::ViewA))
}
