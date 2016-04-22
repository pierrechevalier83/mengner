extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings, clear};

struct Palette {
}

impl Palette {
    fn green() -> [f32; 4] {
        [0.5, 1.0, 0.5, 1.0]
    }
}

fn main() {
    let (w, h) = (800, 600); // px
    let mut window: PistonWindow = WindowSettings::new("Hello, world!", (w, h))
                                       .exit_on_esc(true)
                                       .build()
                                       .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            clear(Palette::green(), g);
        });
    }
}
