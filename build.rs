extern crate image;

use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let input = image::open("data/tiles.gif").unwrap();
    let mut output = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data/tiles.raw")
        .unwrap();
    let mut pixelvec = vec![];
    for pixel in input.to_luma().pixels() {
        pixelvec.push(pixel.data[0]);
        if pixelvec.len() == 8 {
            let mut byte = 0u8;
            for pixel in pixelvec.drain(..) {
                byte = byte << 1;
                if pixel > 127 {
                    byte |= 1;
                }
            }
            output.write_all(&[byte]).unwrap();
        }
    }
    // If the number of pixels isn't a multiple of 8, the last few pixels will be discarded.
}
