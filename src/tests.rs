extern crate serde_json;

use image::GenericImage;
// use std::fs;
use std::io::{Read, Write};
use super::*;

#[test]
fn render_options_modification() {
	assert_eq!(
		OHLCRenderOptions {
			title: String::new(),
			title_colour: 0,
			background_tint: 0xFE,
			current_value_colour: 0x69696968,
			value_prefix: String::new(),
			value_suffix: String::new(),
			time_units: 3600,
			h_axis_options: AxisOptions::new(),
			v_axis_options: AxisOptions::new(),
			down_colour: 0x69696969,
			up_colour: 0x69696970,
		},
		OHLCRenderOptions::new()
			.indicator_colours(0x69696968, 0x69696969, 0x69696970)
			.background_tint(0xFEFEFEFE)
	);
}

#[test]
fn axis_options_modification() {
	assert_eq!(
		AxisOptions {
			title: "I'm a meme".to_string(),
			title_colour: 69,
			line_colour: 70,
			line_frequency: 71.,
			label_colour: 72,
			label_frequency: 73.,
		},
		AxisOptions::new()
			.title("I'm a meme", 69)
			.line(70, 71.)
			.label(72, 73.)
	);
}

#[test]
fn render_draw_sample_data() {
	let mut buf = String::new();
	let _ = File::open("sample_data.json").unwrap().read_to_string(&mut buf);
	let _ = OHLCRenderOptions::new()
		.title("BTCUSD | ohlc-rs", 0x007F7FFF)
		.v_axis(|va| va
			.line(0xCCCCCCFF, 200.)
			.label(0x222222FF, 200.)
		)
		.h_axis(|va| va
			.line(0xD2D2D2FF, 24.)
			.label(0x222222FF, 24.)
		)
		.value_strings("$", "")
		.render_and_save(
			self::serde_json::from_str(&buf).unwrap(),
			&Path::new("test-draw-sample-data.png"),
		);
}

/*
#[test]
fn render_repetition() {
	let _ = OHLCRenderOptions::new()
		.render_and_save(
			vec![OHLC { o: 2.0, h: 4.0, l: 0.0, c: 1.0 }; 168],
			&Path::new("test-repetition.png")
		);
}

#[test]
fn render_draw_v_axis_lines() {
	let _ = OHLCRenderOptions::new()
		.v_axis(|va| va
			.line_colour(0x000000FF)
			.line_frequency(5.)
		)
		.render_and_save(
			vec![OHLC { o: 2.0, h: 12.0, l: 0.0, c: 6.0 }; 168],
			&Path::new("test-draw-lines-vaxis.png")
		);
}

#[test]
fn render_up_down() {
	let _ = OHLCRenderOptions::new()
		.render_and_save(
			vec![
				OHLC { o: 1.0, h: 4.0, l: 0.0, c: 2.0 },
				OHLC { o: 2.0, h: 4.0, l: 0.0, c: 1.0 }
			],
			&Path::new("test-up-down.png")
		);
}

#[test]
fn render_temp_copy() {
	let _ = OHLCRenderOptions::new()
		.render(
			vec![OHLC { o: 2.0, h: 4.0, l: 0.0, c: 1.0 }; 3],
			|path| if let Err(err) = fs::copy(path, &Path::new("test-temp-copy.png")) {
				Err(format!("File copy error: {:?}", err))
			} else {
				Ok(())
			});
}
*/

// Technically not a test, it just generates the fonts array based on the fonts png.
#[test]
fn generate_fonts_file() {
	let img = image::open("consolas-18px-ascii-table.png").unwrap();

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
			if (base_y == 7 && base_x == 15) || (base_y == 2 && base_x == 0) { continue }
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

	let mut f = File::create("src/fonts.rs").unwrap();
	let _ = f.write(output.as_bytes());
}
