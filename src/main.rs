extern crate sdl2;
extern crate time;

mod phi;
mod views;

fn main() {
    ::phi::spawn("ArcadeRS Shooter",
                 |phi| Box::new(::views::ShipView::new(phi)));
}
