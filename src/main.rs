extern crate minifb;
extern crate ticktock;
extern crate rand;

use minifb::{Key, Scale, Window, WindowOptions};

mod tileset;
use tileset::*;

mod effects;
use effects::Sequencer;

mod util;

type Framebuffer = [u32; FB_WIDTH * FB_HEIGHT];
type Tilebuffer = [ColouredTile; WIDTH * HEIGHT];

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

const FB_WIDTH: usize = WIDTH * TILE_WIDTH;
const FB_HEIGHT: usize = HEIGHT * TILE_HEIGHT;

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    r << 16 | g << 8 | b
}

#[derive(Copy, Clone, Debug)]
pub struct ColouredTile {
    index: u8,
    fg: u32,
    bg: u32,
}

struct Demo {
    framebuffer: Framebuffer,
    tilebuffer: Tilebuffer,
    tiles: Tileset,
    sequencer: Sequencer,
    is_running: bool,
    clock: ticktock::Clock,
}

impl Demo {
    fn new() -> Demo {
        Demo {
            framebuffer: [0; FB_WIDTH * FB_HEIGHT],
            tilebuffer: [ColouredTile {
                index: 32,
                fg: rgb(255,255,255),
                bg: rgb(0,0,0),
            }; FB_WIDTH / TILE_WIDTH * FB_HEIGHT / TILE_HEIGHT],
            tiles: gen_tileset(),
            sequencer: Sequencer::new(),
            is_running: true,
            clock: ticktock::Clock::framerate(60.0),
        }
    }

    fn step(&mut self, window: &mut Window) -> (u64, std::time::Instant) {
        if window.is_key_down(Key::Q) || window.is_key_down(Key::Escape) {
            self.is_running = false;
        }
        let (tick, start) = self.clock.wait_until_tick();
        self.sequencer.step(&mut self.tilebuffer, tick);
        self.draw();
        window.update_with_buffer(&self.framebuffer).unwrap();
        (tick, start)
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn draw(&mut self) {
        for (y, row) in self.tilebuffer.chunks(FB_WIDTH / TILE_WIDTH).enumerate() {
            for (x, tile) in row.iter().enumerate() {
                show_tile(
                    &mut self.framebuffer,
                    x,
                    y,
                    self.tiles[tile.index as usize],
                    tile.fg,
                    tile.bg,
                );
            }
        }
    }
}

fn main() {
    let options = WindowOptions {
        scale: Scale::X2,
        ..Default::default()
    };
    let mut window = Window::new("minidemo", FB_WIDTH, FB_HEIGHT, options).unwrap();
    let mut demo = Demo::new();
    let mut previous_time = std::time::Instant::now();
    while window.is_open() && demo.is_running() {
        let (frame, time) = demo.step(&mut window);
        let duration = time - previous_time;
        let duration = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1.0e9;
        // Show the fps every 2 seconds
        if frame % 120 == 0 {
            println!("fps = {}", 120.0 / duration);
            previous_time = time;
        }
    }
}
