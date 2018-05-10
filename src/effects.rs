use {rgb, ColouredTile, Tilebuffer, HEIGHT, WIDTH};

use rand::random;

pub struct Sequencer {
    fire: [u8; WIDTH * HEIGHT],
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            fire: [0; WIDTH * HEIGHT],
        }
    }

    pub fn step(&mut self, tilebuffer: &mut Tilebuffer, time: u64) {
        for x in 0..WIDTH {
            self.fire[x + (HEIGHT - 1) * WIDTH] = random();
        }
        for y in (0..HEIGHT - 1).rev() {
            for x in 1..(WIDTH - 1) {
                self.fire[x + y * WIDTH] = self.fire[x - 1 + (y + 1) * WIDTH] / 3
                    + self.fire[x + (y + 1) * WIDTH] / 3
                    + self.fire[x + 1 + (y + 1) * WIDTH] / 3;
            }
        }
        for (i, bg) in self.fire.iter().enumerate() {
            tilebuffer[i] = ColouredTile {
                tile_index: b' ',
                fg: 0,
                bg: rgb(*bg,20,0),
            };
        }
    }
}
