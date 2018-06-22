use std;

use {rgb, ColouredTile, Tilebuffer, HEIGHT, WIDTH};

use util::{Texture, Vec2, Vec4};

use rand::random;

trait Effect {
    fn step(&mut self, tilebuffer: &mut Tilebuffer, time: u64);
}

pub struct Sequencer {
    flower: Flower,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            flower: Flower::new(),
        }
    }

    pub fn step(&mut self, tilebuffer: &mut Tilebuffer, time: u64) {
        self.flower.step(tilebuffer, time);
    }
}

struct Flower {
    colours: [u32; 10],
    init: bool,
}

impl Flower {
    fn new() -> Flower {
        let mut colours = [0; 10];
        for colour in &mut colours[..] {
            *colour = random();
        }
        Flower {
            colours,
            init: false,
        }
    }
}

impl Effect for Flower {
    fn step(&mut self, buffer: &mut Tilebuffer, time: u64) {
        if !self.init {
            for x in 0..WIDTH / 2 {
                for y in 0..HEIGHT / 2 {
                    buffer[x + y * WIDTH].index = 2;
                }
                for y in HEIGHT / 2..HEIGHT {
                    buffer[x + y * WIDTH].index = 18;
                }
            }
            for x in WIDTH / 2..WIDTH {
                for y in 0..HEIGHT / 2 {
                    buffer[x + y * WIDTH].index = 3;
                }
                for y in HEIGHT / 2..HEIGHT {
                    buffer[x + y * WIDTH].index = 19;
                }
            }
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let ColouredTile {
                        ref mut fg,
                        ref mut bg,
                        ..
                    } = buffer[x + y * WIDTH];
                    *fg = rgb(255, 255, 255);
                    *bg = rgb(0, 0, 0);
                    let checkerboard = x ^ y ^ (x >> 4) ^ (y >> 4);
                    if (checkerboard & 1) != 0 {
                        std::mem::swap(fg, bg);
                    }
                }
            }
            self.init = true;
        }
    }
}
