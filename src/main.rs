extern crate piston_window;

use piston_window::{clear, G2d, Context, PistonWindow, rectangle, WindowSettings};

struct Palette {
}

impl Palette {
    fn green() -> [f32; 4] {
        [0.5, 1.0, 0.5, 1.0]
    }
    fn red() -> [f32; 4] {
        [1.0, 0.5, 0.5, 1.0]
    }
}

#[derive(Debug)]
struct Square {
    dimensions: (u32, u32),
    position: (u32, u32),
}

impl Square {
    fn new() -> Square {
        Square {
            dimensions: (1200, 800),
            position: (0, 0),
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
        window.draw_2d(&e, |mut c, mut g| {
            let mut m = Mengner::new();
            let s = Square::new();
            let order = 5;
            m.process_layers(order, s, &mut c, &mut g);
        });
    }
}

struct Mengner {
    n: u32,
}

impl Mengner {
    fn new() -> Mengner {
        Mengner { n: 3 }
    }

    fn process_layers(&mut self, n_layers: u32, s: Square, c: &mut Context, g: &mut G2d) {
        clear(Palette::green(), g);
        let mut previous = Vec::new();
        previous.push(s);
        for _ in 0..n_layers {
            let next = self.process_layer(&previous, c, g);
            previous = next;
        }
    }

    fn process_layer(&mut self, layer: &Vec<Square>, c: &mut Context, g: &mut G2d) -> Vec<Square> {
        let mut next_layer = Vec::new();
        for s in layer {
            next_layer.extend(self.split_and_punch(s, c, g));
        }
        next_layer
    }

    fn split_and_punch(&mut self, s: &Square, c: & Context, g: &mut G2d) -> Vec<Square> {
        let mut unprocessed = Vec::new();
        let size = self.n;
        for col in 0..size {
            for row in 0..size {
                if self.is_middle(row, col) {
                    Mengner::punch(self.make_subsquare(s, row, col), c, g);
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

    fn punch(s: Square, c: &Context, g: &mut G2d) {
        let (x, y) = s.position;
        let (w, h) = s.dimensions;
        rectangle(Palette::red(), [x as f64, y as f64, w as f64, h as f64], c.transform, g);
    }

    fn make_subsquare(&self, s: &Square, row: u32, col: u32) -> Square {
        Square {
            dimensions: (s.dimensions.0 / self.n, s.dimensions.1 / self.n),
            position: (s.position.0 + row * (s.dimensions.0 / self.n),
                       s.position.1 + col * (s.dimensions.1 / self.n)),
        }
    }
}
