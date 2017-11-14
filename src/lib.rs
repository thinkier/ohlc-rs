#[macro_use]
extern crate serde_derive;
extern crate tempdir;

use tempdir::*;

pub mod data;
pub mod options;

pub use data::*;
pub use options::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::path::*;

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
			down_colour: 0xFFFF0000,
			// Bright-ass green
			up_colour: 0xFF00FF00,
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
		unimplemented!()
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
}
