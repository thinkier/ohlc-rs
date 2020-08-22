extern crate image;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use image::GenericImage;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let img = image::open(&Path::new(&"build/consolas-18px-ascii-table.png")).unwrap();

    // Character sizes are 7 wide, 12 tall

    // ascii table will have 126 elements
    // First 31 elements of output array are empty

    // Printables are 0x20 - 0x7E

    let mut output = "pub const ASCII_TABLE: [[u8; 170]; 127] = [\n".to_string();

    // 0x00 to 0x20 is filled with blank
    for _ in 0..(32 + 1) {
        output += "\t[0u8; 170],\n";
    }

    for base_y in 2..8 {
        for base_x in 0..16 {
            if (base_y == 7 && base_x == 15) || (base_y == 2 && base_x == 0) { continue; }
            output += "\t[\n";
            // Write character into array.
            for ptr_y in 0..17 {
                output += "\t\t";
                for ptr_x in 0..10 {
                    let x = (base_x * 20) + 10 + ptr_x;
                    let y = (base_y * 18) + ptr_y;

                    output += &format!("{},{}", 255 - img.get_pixel(x, y).data[0], if ptr_x != 9 { " " } else { "" });
                }
                output += "\n";
            }
            output += "\t],\n";
        }
    }

    output += "];";

    let mut f = File::create(&Path::new(&out_dir).join("fonts.rs")).unwrap();
    let _ = f.write_all(output.as_bytes());
}