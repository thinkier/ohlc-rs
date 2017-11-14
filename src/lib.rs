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
	title: String,
	/// Colour for the title of the chart
	text_colour: u32,
	/// The prefix for the values represented in the OHLC
	value_prefix: String,
	/// The suffix for the values represented in the OHLC
	value_suffix: String,
	/// The amount of time, in seconds, each OHLC objects represent
	time_units: u64,
	/// Options for the horizontal axis
	h_axis_options: AxisOptions,
	/// Options for the vertical axis
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

	/// Renders the OHLC Chart by the data, using the configs provided.
	///
	/// Takes a lambda function for processing the image once it's rendered, do not do anything asynchronous with the image as it will be deleted as soon as the function finishes.
	///
	/// Returns an error string originating from OHLC if an error occurs, and the result of the callback function otherwise.
	pub fn render<F>(self, data: Vec<OHLC>, callback: F) -> Result<Result<(), String>, String>
		where F: Fn(&Path) -> Result<(), String> + Sized {
		// Create temporary directory: mostly copied example from https://github.com/rust-lang-nursery/tempdir
		if let Ok(dir) = TempDir::new(&format!("ohlc_render_{:?}", data.hash(&mut DefaultHasher::new()))) {
			let file_path = dir.path().join("chart.png");

			//			let mut f = File::create(file_path)?;
			//			f.write_all(b"Hello, world!")?;
			//			f.sync_all()?;
			//			dir.close()?;

			// Render chart and save to temporary directory

			let result = (callback)(&file_path);

			// Delete temporary directory

			Ok(result)
		} else {
			Err("Failed to create a temporary directory.".to_string())
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
