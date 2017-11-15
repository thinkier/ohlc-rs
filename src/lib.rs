#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate tempdir;


use tempdir::*;

pub mod data;
pub mod options;
mod utils;

pub use data::*;
pub use options::*;
use utils::*;

use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::*;

use image::{ImageBuffer, Pixel};

/// OHLC Chart Configuration, mutate through the methods
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OHLCRenderOptions {
	/// Title of the chart
	/// Currently ignored
	title: String,
	/// Currently ignored
	/// Colour for the title of the chart
	text_colour: u32,
	/// The prefix for the values represented in the OHLC
	/// Currently ignored
	value_prefix: String,
	/// The suffix for the values represented in the OHLC
	/// Currently ignored
	value_suffix: String,
	/// The amount of time, in seconds, each OHLC objects represent
	/// Currently ignored
	time_units: u64,
	/// Options for the horizontal axis
	/// Currently ignored
	h_axis_options: AxisOptions,
	/// Options for the vertical axis
	/// Currently ignored
	v_axis_options: AxisOptions,
	/// RGBA(8) Colour for when the OHLC indicates fall
	down_colour: u32,
	/// RGBA(8) Colour for when the OHLC indicates rise
	up_colour: u32,
}

impl OHLCRenderOptions {
	/// Creates an object for render options with default parameters
	pub fn new() -> OHLCRenderOptions {
		OHLCRenderOptions {
			title: String::new(),
			text_colour: 0,
			value_prefix: String::new(),
			value_suffix: String::new(),
			// Default is 1 hour
			time_units: 3600,
			h_axis_options: AxisOptions::new(),
			v_axis_options: AxisOptions::new(),
			// Bright-ass red
			down_colour: 0xFF0000FF,
			// Bright-ass green
			up_colour: 0x00FF00FF,
		}
	}

	pub fn colours(mut self, down: u32, up: u32) -> Self {
		self.down_colour = down;
		self.up_colour = up;

		self
	}

	/// Renders the OHLC Chart by the data, using the configs provided.
	///
	/// Takes a lambda function for processing the image once it's rendered, do not do anything asynchronous with the image as it will be deleted as soon as the function finishes.
	///
	/// Returns an error string originating from OHLC if an error occurs, and the result of the callback function otherwise.
	pub fn render<F>(&self, data: Vec<OHLC>, callback: F) -> Result<Result<(), String>, String>
		where F: Fn(&Path) -> Result<(), String> + Sized {
		let mut hasher = DefaultHasher::new();
		data.hash(&mut hasher);

		// Create temporary directory
		if let Ok(dir) = TempDir::new(&format!("ohlc_render_{}", hasher.finish())) {
			let file_path = dir.path().join("chart.png");

			let mut result = match self.render_and_save(data, &file_path) {
				Ok(_) => Ok((callback)(&file_path)),
				Err(err) => Err(err)
			};

			let _ = dir.close(); // Delete temporary directory

			result
		} else {
			Err("Failed to create a temporary directory.".to_string())
		}
	}

	/// Renders the chart and saves it to the specified path
	///
	/// Returns an error string if an error occurs
	pub fn render_and_save(&self, data: Vec<OHLC>, path: &Path) -> Result<(), String> {
		let ohlc_of_set = calculate_ohlc_of_set(&data);

		let margin = 40u32;

		let width = 780;
		let height = 350;

		let mut image_buffer: ImageBuffer<image::Rgba<u8>, _> = ImageBuffer::new(width, height);

		let candle_width = (width - (2 * margin)) as f64 / data.len() as f64;
		let stick_width = (|x| if x < 1 { x } else { x })((candle_width / 20.).round() as u32);

		let y_val_increment = ohlc_of_set.range() / (height - (2 * margin)) as f64;

		for (i, ohlc_elem) in data.iter().enumerate() {
			let colour = if ohlc_elem.o > ohlc_elem.c { self.down_colour } else { self.up_colour };

			// Yes, no left margin
			let begin_pos = (candle_width * i as f64).round() as u32;
			let end_pos = (candle_width * (i + 1) as f64).round() as u32;

			for y_state in (((ohlc_elem.c - ohlc_elem.l) / y_val_increment).round() as u32)..(((ohlc_elem.o - ohlc_elem.l) / y_val_increment).round() as u32) {
				let y = height - y_state - margin;
				for x in begin_pos..(end_pos + 1) {
					let mut chs = image_buffer
						.get_pixel_mut(x, y)
						.channels_mut();
					for j in 0..4 {
						chs[3 - j] = (colour >> (8 * j)) as u8;
					}
				}
			}

			{
				let x_center = (((begin_pos + end_pos) as f64) / 2.).round() as u32;
				for y_state in (((ohlc_elem.l - ohlc_elem.l) / y_val_increment).round() as u32)..(((ohlc_elem.h - ohlc_elem.l) / y_val_increment).round() as u32) {
					let y = height - y_state - margin;

					for x in (x_center - stick_width) as u32..(x_center + stick_width) as u32 {
						let mut chs = image_buffer
							.get_pixel_mut(x, y)
							.channels_mut();
						for j in 0..4 {
							chs[3 - j] = (colour >> (8 * j)) as u8;
						}
					}
				}
			}
		}

		match File::create(path) {
			Ok(ref mut file) => match image::ImageRgba8(image_buffer).save(file, image::PNG) {
				Ok(_) => Ok(()),
				Err(err) => Err(format!("Image write error: {:?}", err))
			}
			Err(err) => Err(format!("File create error: {:?}", err))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn modifiers() {
		assert_eq!(
			OHLCRenderOptions {
				title: String::new(),
				text_colour: 0,
				value_prefix: String::new(),
				value_suffix: String::new(),
				time_units: 3600,
				h_axis_options: AxisOptions::new(),
				v_axis_options: AxisOptions::new(),
				down_colour: 0x69696969,
				up_colour: 0x69696969,
			},
			OHLCRenderOptions::new()
				.colours(0x69696969, 0x69696969)
		);
	}

	#[test]
	fn render_simple() {
		let _ = OHLCRenderOptions::new()
			.render_and_save(vec![OHLC {
				o: 1.0,
				h: 2.0,
				l: 0.0,
				c: 1.0,
			}, OHLC {
				o: 2.0,
				h: 4.0,
				l: 0.0,
				c: 1.0,
			}], &Path::new("test.png"));
	}
}
