use Framebuffer;
use FB_WIDTH;

pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;

const TILESET_WIDTH: usize = 16;
const TILESET_HEIGHT: usize = 16;
const PIXELS_PER_BYTE: usize = 8;

pub type Tile = [bool; TILE_WIDTH * TILE_HEIGHT];
pub type Tileset = [Tile; TILESET_WIDTH * TILESET_HEIGHT];

const RAW_TILES: &'static [u8; TILESET_WIDTH * TILESET_HEIGHT * TILE_WIDTH * TILE_HEIGHT
             / PIXELS_PER_BYTE] = include_bytes!("../data/tiles.raw");

fn tile_pixel(x: usize, y: usize) -> bool {
    let xlow = x & 0x07;
    let xhigh = x >> 3;
    ((RAW_TILES[xhigh + y * TILESET_WIDTH] << xlow) & 0x80) != 0
}

pub fn gen_tileset() -> Tileset {
    let mut tiles = [[false; TILE_WIDTH * TILE_HEIGHT]; TILESET_WIDTH * TILESET_HEIGHT];
    for y in 0..128 {
        for x in 0..128 {
            let (xh, yh) = (x / 8, y / 8);
            let (xl, yl) = (x & 0x07, y & 0x07);
            tiles[xh + yh * TILESET_WIDTH][xl + yl * TILE_WIDTH] = tile_pixel(x, y);
        }
    }
    tiles
}

pub fn show_tile(
    buffer: &mut Framebuffer,
    mut x: usize,
    mut y: usize,
    tile: Tile,
    fg: u32,
    bg: u32,
) {
    x *= TILE_WIDTH;
    y *= TILE_HEIGHT;
    for tx in 0..TILE_WIDTH {
        for ty in 0..TILE_HEIGHT {
            buffer[x + tx + (y + ty) * FB_WIDTH] = if tile[tx + ty * TILE_WIDTH] { fg } else { bg };
        }
    }
}
