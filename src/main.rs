extern crate piston_window;

use piston_window::{clear, G2d, PistonWindow, WindowSettings};

struct Palette {
}

impl Palette {
    fn green() -> [f32; 4] {
        [0.5, 1.0, 0.5, 1.0]
    }
}

struct Square {
    dimensions: (u32, u32),
    position: (u32, u32),
}

impl Square {
    fn new() -> Square {
        Square {
            dimensions: (800, 600),
            position:(0, 0),
        }
    }
}

fn main() {
    let s = Square::new();
    let mut window: PistonWindow = WindowSettings::new("Hello, world!", s.dimensions)
                                       .exit_on_esc(true)
                                       .build()
                                       .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            clear(Palette::green(), g);
        });
    }
}

struct Mengner {
    n : u32
}

impl Mengner {
    fn new() -> Mengner {
        Mengner {
            n: 3,
        }
    }

    fn process_layer(&mut self, layer: &Vec<Square>) -> Vec<Square> {
        let mut next_layer = Vec::new();
        for s in layer {
            next_layer.extend(self.split_and_punch(s));
        }
        next_layer
    }

    fn split_and_punch(&mut self, s :&Square) -> Vec<Square> {
        let mut unprocessed = Vec::new();
        let size = self.n;
        for col in 0..size {
            for row in 0..size {
                if self.is_middle(row, col) {
                    let s = self.make_subsquare(s, row, col);
                    self.punch(&s);
                } else {
                    unprocessed.push(self.make_subsquare(s, row, col));
                }
            }
        }
        unprocessed
    }

    fn is_middle(&self, row: u32, col: u32) -> bool {
        (col == row) && (col == self.n / 2)
    }

    fn punch(&mut self, s: &Square) {
        // TODO: clear that square
    }

    fn make_subsquare(&self, s: &Square, row: u32, col: u32) -> Square {
        Square {
            dimensions: (s.dimensions.0 / self.n, s.dimensions.1 / self.n),
            position: (col * (s.dimensions.0 / self.n), row * (s.dimensions.1 / self.n))
        }
    }
}
