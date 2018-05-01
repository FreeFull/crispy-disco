extern crate game_time;
extern crate minifb;

use game_time::framerate::RunningAverageSampler;
use game_time::step::VariableStep;
use game_time::{FrameCount, FrameCounter, GameClock};
use minifb::{Key, Scale, Window, WindowOptions};

mod tileset;
use tileset::*;

type Framebuffer = [u32; FB_WIDTH * FB_HEIGHT];

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
struct ColouredTile {
    tile_index: u8,
    fg: u32,
    bg: u32,
}

struct Demo {
    framebuffer: Framebuffer,
    tilebuffer: [ColouredTile; WIDTH * HEIGHT],
    tiles: Tileset,
    is_running: bool,
    clock: GameClock,
    counter: FrameCounter<RunningAverageSampler>,
}

impl Demo {
    fn new() -> Demo {
        Demo {
            framebuffer: [0; FB_WIDTH * FB_HEIGHT],
            tilebuffer: [ColouredTile {
                tile_index: 0,
                fg: 0x00FFFFFF,
                bg: 0,
            }; FB_WIDTH / TILE_WIDTH * FB_HEIGHT / TILE_HEIGHT],
            tiles: gen_tileset(),
            is_running: true,
            clock: GameClock::new(),
            counter: FrameCounter::new(60.0, RunningAverageSampler::with_max_samples(60)),
        }
    }

    fn step(&mut self, window: &mut Window) {
        let time = self.clock.tick(&VariableStep::new());
        self.counter.tick(&time);
        self.draw();
        self.tilebuffer = [ColouredTile {
            tile_index: 16,
            bg: 0x00FFFFFF,
            fg: 0,
        }; FB_WIDTH / TILE_WIDTH * FB_HEIGHT / TILE_HEIGHT];
        for (i, tile) in self.tilebuffer.iter_mut().enumerate() {
            let x = ((i % (FB_WIDTH / TILE_WIDTH)) * 8) as u8;
            let y = ((i / (FB_WIDTH / TILE_WIDTH)) * 8) as u8;
            tile.fg = rgb(x, y, x / 2 + y / 2);
        }
        window.update_with_buffer(&self.framebuffer).unwrap();
        if window.is_key_down(Key::Q) || window.is_key_down(Key::Escape) {
            self.is_running = false;
        }
        self.clock.sleep_remaining(&self.counter);
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
                    self.tiles[tile.tile_index as usize],
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
    while window.is_open() && demo.is_running() {
        demo.step(&mut window);
        if demo.clock.current_frame_number() % 60 == 1 {
            println!("fps: {}", demo.counter.average_frame_rate());
        }
    }
}
