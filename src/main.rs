extern crate game_time;
extern crate minifb;

use game_time::framerate::RunningAverageSampler;
use game_time::step::VariableStep;
use game_time::{FrameCount, FrameCounter, GameClock};
use minifb::{Key, Scale, Window, WindowOptions};

mod tileset;
use tileset::*;

type Framebuffer = [u32; WIDTH * HEIGHT];

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    r << 16 | g << 8 | b
}

struct Demo {
    framebuffer: Framebuffer,
    tiles: Tileset,
    is_running: bool,
    clock: GameClock,
    counter: FrameCounter<RunningAverageSampler>,
}

impl Demo {
    fn new() -> Demo {
        Demo {
            framebuffer: [0; WIDTH * HEIGHT],
            tiles: gen_tileset(),
            is_running: true,
            clock: GameClock::new(),
            counter: FrameCounter::new(60.0, RunningAverageSampler::with_max_samples(60)),
        }
    }

    fn step(&mut self, window: &mut Window) {
        let time = self.clock.tick(&VariableStep::new());
        self.counter.tick(&time);
        self.draw((time.frame_number()) as usize);
        window.update_with_buffer(&self.framebuffer).unwrap();
        if window.is_key_down(Key::Q) {
            self.is_running = false;
        }
        self.clock.sleep_remaining(&self.counter);
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn draw(&mut self, _time: usize) {
        for y in 0..16 {
            for x in 0..16 {
                show_tile(
                    &mut self.framebuffer,
                    x,
                    y,
                    self.tiles[x + y * 16],
                    rgb(255, 255, 255),
                    rgb(0, 128, 255),
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
    let mut window = Window::new("minidemo", WIDTH, HEIGHT, options).unwrap();
    let mut demo = Demo::new();
    while window.is_open() && demo.is_running() {
        demo.step(&mut window);
        if demo.clock.current_frame_number() % 60 == 1 {
            println!("fps: {}", demo.counter.average_frame_rate());
        }
    }
}
