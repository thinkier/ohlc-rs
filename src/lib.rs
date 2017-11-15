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
	pub(crate) title: String,
	/// Currently ignored
	/// Colour for the title of the chart
	pub(crate) title_colour: u32,
	/// Background colour of the entire chart
	pub(crate) background_colour: u32,
	/// The prefix for the values represented in the OHLC
	/// Currently ignored
	pub(crate) value_prefix: String,
	/// The suffix for the values represented in the OHLC
	/// Currently ignored
	pub(crate) value_suffix: String,
	/// The amount of time, in seconds, each OHLC objects represent
	/// Currently ignored
	pub(crate) time_units: u64,
	/// Options for the horizontal axis
	/// Currently ignored
	pub(crate) h_axis_options: AxisOptions,
	/// Options for the vertical axis
	pub(crate) v_axis_options: AxisOptions,
	/// RGBA(8) Colour for when the OHLC indicates fall
	pub(crate) down_colour: u32,
	/// RGBA(8) Colour for when the OHLC indicates rise
	pub(crate) up_colour: u32,
}

impl OHLCRenderOptions {
	/// Creates an object for render options with default parameters
	pub fn new() -> OHLCRenderOptions {
		OHLCRenderOptions {
			title: String::new(),
			title_colour: 0,
			background_colour: 0xDDDDDDFF,
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

	pub fn indicator_colours(mut self, down: u32, up: u32) -> Self {
		self.down_colour = down;
		self.up_colour = up;

		self
	}

	pub fn background_colour(mut self, colour: u32) -> Self {
		self.background_colour = colour;

		self
	}

	pub fn h_axis<F>(mut self, mut f: F) -> Self
		where F: FnMut(AxisOptions) -> AxisOptions {
		self.h_axis_options = (f)(self.h_axis_options);

		self
	}

	pub fn v_axis<F>(mut self, mut f: F) -> Self
		where F: FnMut(AxisOptions) -> AxisOptions {
		self.v_axis_options = (f)(self.v_axis_options);

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
		if let Err(err) = validate(&data) {
			return Err(format!("Data validation error: {}", err));
		}

		let ohlc_of_set = calculate_ohlc_of_set(&data);

		let margin = 45u32;

		let width = 1280;
		let height = 720;

		let mut image_buffer: ImageBuffer<image::Rgba<u8>, _> = ImageBuffer::new(width, height);

		if self.background_colour % 256 > 0 {
			for x in 0..width {
				for y in 0..height {
					let mut chs = image_buffer
						.get_pixel_mut(x, y)
						.channels_mut();
					for j in 0..4 {
						chs[3 - j] = (self.background_colour >> (8 * j)) as u8;
					}
				}
			}
		}

		let candle_width = (width - (2 * margin)) as f64 / data.len() as f64;
		let stick_width = (|x| if x < 1 && candle_width >= 3. { 1 } else { x })((candle_width / 10. + 0.3).round() as u32);

		let y_val_increment = ohlc_of_set.range() / (height - (2 * margin)) as f64;

		if self.v_axis_options.line_colour % 256 > 0 && self.v_axis_options.line_frequency > 0. {
			for y_es in 0..(height - 2 * margin) {
				if (|d| d < y_val_increment && d >= 0.)((ohlc_of_set.h - y_es as f64 * y_val_increment) % self.v_axis_options.line_frequency) {
					let y = y_es + margin;
					for x in 0..(width - 2 * margin) {
						let mut chs = image_buffer
							.get_pixel_mut(x, y)
							.channels_mut();
						for j in 0..4 {
							chs[3 - j] = (self.v_axis_options.line_colour >> (8 * j)) as u8;
						}
					}

					// TODO Use the y here as the anchor for inserting the labels
				}
			}
		}

		for (i, ohlc_elem) in data.iter().enumerate() {
			let colour = if ohlc_elem.o > ohlc_elem.c { self.down_colour } else { self.up_colour };

			// Yes, no left margin
			let begin_pos = (candle_width * i as f64).round() as u32;
			let end_pos = (candle_width * (i + 1) as f64).round() as u32;

			let open_ys = ((ohlc_elem.o - ohlc_of_set.l) / y_val_increment).round() as u32;
			let close_ys = ((ohlc_elem.c - ohlc_of_set.l) / y_val_increment).round() as u32;

			for y_state in if open_ys > close_ys { close_ys..open_ys } else { open_ys..close_ys } {
				let y = height - y_state - margin;
				// Introduce right padding if the candle isn't too short
				for x in begin_pos..(if end_pos - begin_pos > 3 { end_pos - 1 } else { end_pos + 1 }) {
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
				for y_state in (((ohlc_elem.l - ohlc_of_set.l) / y_val_increment).round() as u32)..(((ohlc_elem.h - ohlc_of_set.l) / y_val_increment).round() as u32) {
					let y = height - y_state - margin;

					for x in (x_center - stick_width - 1) as u32..(x_center + stick_width - 1) as u32 {
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

fn validate(data: &Vec<OHLC>) -> Result<(), &'static str> {
	for elem in data {
		return if elem.o > elem.h {
			Err("Opening value is higher than high value.")
		} else if elem.c > elem.h {
			Err("Closing value is higher than high value.")
		} else if elem.l > elem.h {
			Err("Low value is higher than high value.")
		} else if elem.o < elem.l {
			Err("Opening value is lower than low value.")
		} else if elem.c < elem.l {
			Err("Closing value is lower than low value.")
		} else {
			continue
		};
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::fs;
	use super::*;

	#[test]
	fn render_options_modification() {
		assert_eq!(
			OHLCRenderOptions {
				title: String::new(),
				title_colour: 0,
				background_colour: 0xFEFEFEFE,
				value_prefix: String::new(),
				value_suffix: String::new(),
				time_units: 3600,
				h_axis_options: AxisOptions::new(),
				v_axis_options: AxisOptions::new(),
				down_colour: 0x69696969,
				up_colour: 0x69696969,
			},
			OHLCRenderOptions::new()
				.indicator_colours(0x69696969, 0x69696969)
				.background_colour(0xFEFEFEFE)
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
				.title("I'm a meme")
				.title_colour(69)
				.line_colour(70)
				.line_frequency(71.)
				.label_colour(72)
				.label_frequency(73.)
		);
	}

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
}
