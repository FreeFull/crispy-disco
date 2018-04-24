use Framebuffer;
use WIDTH;

pub type Tile = [bool; 8 * 8];
pub type Tileset = [Tile; 16 * 16];

const RAW_TILES: &'static [u8; 2048] = include_bytes!("../data/tiles.raw");
fn tile_pixel(x: usize, y: usize) -> bool {
    let xlow = x & 0x07;
    let xhigh = x >> 3;
    ((RAW_TILES[xhigh + y * 16] << xlow) & 0x80) == 0
}

pub fn gen_tileset() -> Tileset {
    let mut tiles = [[false; 8 * 8]; 16 * 16];
    for y in 0..128 {
        for x in 0..128 {
            let (xh, yh) = (x / 8, y / 8);
            let (xl, yl) = (x & 0x07, y & 0x07);
            tiles[xh + yh * 16][xl + yl * 8] = tile_pixel(x, y);
        }
    }
    tiles
}

pub fn show_tile(buffer: &mut Framebuffer, mut x: usize, mut y: usize, tile: Tile, fg: u32, bg: u32) {
    x *= 8;
    y *= 8;
    for tx in 0..8 {
        for ty in 0..8 {
            buffer[x + tx + (y + ty) * WIDTH] = if tile[tx + ty * 8] { fg } else { bg };
        }
    }
}