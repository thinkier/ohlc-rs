extern crate image;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate tempdir;

use api::RendererExtension;
pub use data::*;
use model::*;
use model::basic_indicative_lines::BasicIndicativeLines;
use model::grid_lines::GridLines;
use model::ohlc_candles::OHLCCandles;
pub use options::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::*;
use std::time::SystemTime;
use tempdir::*;
pub use utils::*;

pub mod api;
pub mod data;
mod fonts;
pub mod model;
pub mod options;
#[cfg(test)]
mod tests;
pub mod utils;

/// OHLC Chart Configuration, mutate through the methods
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OHLCRenderOptions<RE: RendererExtension + Sized> {
	/// Title of the chart
	pub title: String,
	/// Colour for the title of the chart
	pub title_colour: u32,
	/// Background tint of the entire chart (the tint is the value for all of R, G and B)
	pub background_colour: u32,
	/// Colour for the "current value" dot and line across the chart
	pub current_value_colour: u32,
	/// The prefix for the values represented in the OHLC
	pub value_prefix: String,
	/// The suffix for the values represented in the OHLC
	pub value_suffix: String,
	/// The amount of time, in seconds, each OHLC objects represent
	pub time_units: u64,
	/// Options for the horizontal axis
	pub h_axis_options: AxisOptions,
	/// Options for the vertical axis
	pub v_axis_options: AxisOptions,
	/// RGBA(8) Colour for when the OHLC indicates fall
	pub down_colour: u32,
	/// RGBA(8) Colour for when the OHLC indicates rise
	pub up_colour: u32,
	/// Render extension
	pub(crate) render_extension: RE,
}

impl<RE: RendererExtension> OHLCRenderOptions<RE> {
	/// Creates an object for render options with default parameters
	pub fn new(render_extension: RE) -> OHLCRenderOptions<RE> {
		OHLCRenderOptions {
			title: String::new(),
			title_colour: 0,
			background_colour: 0xDDDDDDFF,
			current_value_colour: 0x2E44EAFF,
			value_prefix: String::new(),
			value_suffix: String::new(),
			// Default is 1 hour
			time_units: 3600,
			h_axis_options: AxisOptions::new(),
			v_axis_options: AxisOptions::new(),
			down_colour: 0xD33040FF,
			up_colour: 0x27A819FF,
			render_extension,
		}
	}

	pub fn title(mut self, title: &str, colour: u32) -> Self {
		self.title = title.to_string();
		self.title_colour = colour;

		self
	}

	pub fn indicator_colours(mut self, current_val: u32, down: u32, up: u32) -> Self {
		self.current_value_colour = current_val;
		self.down_colour = down;
		self.up_colour = up;

		self
	}

	pub fn value_strings(mut self, prefix: &str, suffix: &str) -> Self {
		self.value_prefix = prefix.to_string();
		self.value_suffix = suffix.to_string();

		self
	}

	pub fn background_colour(mut self, colour: u32) -> Self {
		self.background_colour = colour;

		self
	}

	pub fn time_units(mut self, time_units: u64) -> Self {
		self.time_units = time_units;

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
		let start_time = SystemTime::now();

		if let Err(err) = validate(&data) {
			return Err(format!("Data validation error: {}", err));
		}

		#[cfg(test)] {
			debug!("Validated input data @ {:?}", start_time.elapsed());
		}

		let ohlc_of_set = calculate_ohlc_of_set(&data[..]);

		let margin_top = 60;
		let margin_bottom = 35;
		let margin_left = 12;
		let margin_right = 113;

		let width = 1310;
		let height = 650;

		#[cfg(test)] {
			debug!("Initialized variables @ {:?}", start_time.elapsed());
		}

		let mut image_buffer = vec![(self.background_colour >> 24) as u8; width * height * 3];

		#[cfg(test)] {
			debug!("Allocated vector @ {:?}", start_time.elapsed());
		}

		{
			let r = (self.background_colour >> 24) as u8;
			let g = (self.background_colour >> 16) as u8;
			let b = (self.background_colour >> 8) as u8;

			if r != g || g != b {
				let colours = [r, g, b];

				for y in 0..height {
					for x in 0..width {
						for j in 0..3 {
							image_buffer[(x + y * width) * 3 + j] = colours[j];
						}
					}
				}
			}
		}

		#[cfg(test)] {
			debug!("Populated background @ {:?}", start_time.elapsed());
		}

		{
			let mut chart_buffer = ChartBuffer::new(width, height, Margin {
				top: margin_top,
				bottom: margin_bottom,
				left: margin_left,
				right: margin_right,
			}, ohlc_of_set.h, ohlc_of_set.l, (self.time_units * data.len() as u64) as i64, self.background_colour, &mut image_buffer[..]);

			GridLines::new(
				self.h_axis_options.line_colour,
				true,
				200.,
				86400).apply(&mut chart_buffer, &data[..]);

			#[cfg(test)] {
				debug!("Rendered grid lines @ {:?}", start_time.elapsed());
			}

			OHLCCandles::new(self.up_colour, self.down_colour).apply(&mut chart_buffer, &data[..]);

			#[cfg(test)] {
				debug!("Rendered candles @ {:?}", start_time.elapsed());
			}

			BasicIndicativeLines::new(self.up_colour, self.down_colour, self.current_value_colour).apply(&mut chart_buffer, &data[..]);

			#[cfg(test)] {
				debug!("Rendered basic indicator lines @ {:?}", start_time.elapsed());
			}

			chart_buffer.text((8,8), &self.title, self.title_colour);

			#[cfg(test)] {
				debug!("Added title text @ {:?}", start_time.elapsed());
			}

			self.render_extension.apply(&mut chart_buffer, &data[..]);

			#[cfg(test)] {
				debug!("Rendered extension:{} @ {:?}", self.render_extension.name(), start_time.elapsed());
			}
		}

		#[cfg(test)] {
			debug!("Completed all rendering @ {:?}", start_time.elapsed());
		}

		// File save occurs here
		if let Err(err) = image::save_buffer(path, &image_buffer[..], width as u32, height as u32, image::RGB(8)) {
			Err(format!("Image write error: {:?}", err))
		} else {
			#[cfg(test)] {
				debug!("Chart PNG compression finished {:?}", start_time.elapsed());
			}

			debug!("Chart rendered in {:?}", start_time.elapsed());

			Ok(())
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
			continue;
		};
	}
	Ok(())
}
