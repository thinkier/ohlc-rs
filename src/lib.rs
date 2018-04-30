extern crate image;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate tempdir;

use api::RendererExtension;
pub use data::*;
use model::*;
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
pub struct OHLCRenderOptions {
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
	/// List of extensions to render
	#[serde(skip)]
	render_extensions: Vec<&'static RendererExtension>,
}

impl OHLCRenderOptions {
	/// Creates an object for render options with default parameters
	pub fn new() -> OHLCRenderOptions {
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
			render_extensions: vec![],
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

	pub fn add_renderer<RE: RendererExtension + Sized>(mut self, re: &'static RE) -> Self {
		self.render_extensions.push(re);

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

		// String.bytes, top edge x, leftmost edge y, colour, do a border
		let mut text_renders: Vec<(Vec<u8>, usize, usize, u32, bool)> = vec![];

		let ohlc_of_set = calculate_ohlc_of_set(&data);

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

		// Width of the "candle" in the candlestick chart
		let candle_width = ((width - (margin_left + margin_right)) as f64 / data.len() as f64).floor();
		// Width of the "stick" component in the candlestick chart
		let stick_width = (|x| if x < 1 || candle_width <= 5. { 1 } else { x })((candle_width / 10. + 0.3).round() as usize);

		// Defines how much the Y value should increment for every unit of the OHLC supplied
		let y_val_increment = ohlc_of_set.range() / (height - (margin_top + margin_bottom)) as f64;

		#[cfg(test)] {
			debug!("Calculated candle data @ {:?}", start_time.elapsed());
		}

		// Rendering the horizontal (price) lines occur here
		if self.v_axis_options.line_colour % 256 > 0 && self.v_axis_options.line_frequency > 0. {
			for y_es in 0..(height - (margin_top + margin_bottom)) {
				if (|d| d < y_val_increment && d >= 0.)((ohlc_of_set.h - y_es as f64 * y_val_increment) % self.v_axis_options.line_frequency) {
					let y = y_es + margin_top;
					for x in margin_left..(width - margin_right) {
						colour_rgba(&mut image_buffer, width, x, y, self.v_axis_options.line_colour);
					}
				}

				// Rendering text for the lines occur here
				if self.v_axis_options.label_colour % 256 > 0 && (|d| d < y_val_increment && d >= 0.)((ohlc_of_set.h - y_es as f64 * y_val_increment) % self.v_axis_options.label_frequency) {
					let base_y = y_es + margin_top - 8; // Top edge...

					let mut chars = format!("{}{:.8}{}", self.value_prefix, ((ohlc_of_set.h - y_es as f64 * y_val_increment) / self.v_axis_options.label_frequency).round() * self.v_axis_options.label_frequency, self.value_suffix).into_bytes();

					while chars.len() > ((margin_right as f32 - 10.) / 10.).floor() as usize {
						let _ = chars.pop();
					}
					text_renders.push((chars, width - margin_right + 10, base_y, self.v_axis_options.label_colour, false))
				}
			}
		}

		#[cfg(test)] {
			debug!("Rendered horizontal lines (price ticks) @ {:?}", start_time.elapsed());
		}

		// Rendering the vertical (time) lines occur here
		if self.h_axis_options.line_colour % 256 > 0 && self.h_axis_options.line_frequency > 0. {
			let data_len = data.len();

			let line_count = (data_len as f64 / self.h_axis_options.line_frequency).round() as usize + 1;
			let line_interval = (candle_width * self.h_axis_options.line_frequency).round() as usize;
			let label_count = (data_len as f64 / self.h_axis_options.label_frequency).round() as usize + 1;
			let label_interval = (candle_width * self.h_axis_options.label_frequency).round() as usize;

			for x_idx in 0..line_count {
				let x = x_idx * line_interval + margin_left;

				for y in margin_top..(height - margin_bottom) {
					colour_rgba(&mut image_buffer, width, x, y, self.h_axis_options.line_colour);
				}
			}

			// Rendering text for the lines occur here
			for x_idx in 0..label_count {
				let x = x_idx * label_interval + margin_left;

				if self.h_axis_options.label_colour % 256 > 0 && self.h_axis_options.label_frequency > 0. {
					let mut chars = duration_string((self.time_units as f64 * self.h_axis_options.label_frequency * (label_count - x_idx - 1) as f64).round() as u64).into_bytes();

					while chars.len() > ((margin_right as f32 - 10.) / 10.).floor() as usize {
						let _ = chars.pop();
					}
					text_renders.push((chars, x - 10, height - margin_bottom + 5, self.h_axis_options.label_colour, false))
				}
			}
		}

		#[cfg(test)] {
			debug!("Rendered vertical lines (time ticks) @ {:?}", start_time.elapsed());
		}

		// The below section renders the OHLC candles
		for (i, ohlc_elem) in data.iter().enumerate() {
			let colour = if ohlc_elem.o > ohlc_elem.c { self.down_colour } else { self.up_colour };

			// Yes, no left margin
			let begin_pos = (candle_width * i as f64) as usize + margin_left;
			let end_pos = (candle_width * (i + 1) as f64) as usize + margin_left;

			let open_ys = ((ohlc_elem.o - ohlc_of_set.l) / y_val_increment).round() as usize;
			let close_ys = ((ohlc_elem.c - ohlc_of_set.l) / y_val_increment).round() as usize;

			let x_center = (((begin_pos + end_pos) as f64) / 2.).round() as usize;

			// Candles are rendered inside here
			for y_state in if open_ys > close_ys { close_ys..(1 + open_ys) } else { open_ys..(1 + close_ys) } {
				let y = height - y_state - margin_bottom;
				// Introduce right padding if the candle isn't too short
				for x in begin_pos..(if end_pos - begin_pos > 3 { end_pos - 1 } else { end_pos + 1 }) {
					colour_rgba(&mut image_buffer, width, x, y, colour);
				}
			}

			// Sticks and rendered inside here
			for y_state in (((ohlc_elem.l - ohlc_of_set.l) / y_val_increment).round() as usize)..(1 + ((ohlc_elem.h - ohlc_of_set.l) / y_val_increment).round() as usize) {
				let y = height - y_state - margin_bottom;

				for x in (x_center - stick_width - 1) as usize..(x_center + stick_width - 1) as usize {
					colour_rgba(&mut image_buffer, width, x, y, colour);
				}
			}

			// Render the star for the current price line
			if i == data.len() - 1 {
				let y = height - (((ohlc_of_set.c - ohlc_of_set.l) / y_val_increment).round() as usize) - margin_bottom;
				for x_offset in -2i32..3 {
					for y_offset in -2i32..3 {
						if !(x_offset == y_offset || x_offset + y_offset == 0 || x_offset == 0) { continue; }

						colour_rgba(&mut image_buffer, width, (x_offset + (x_center as i32)) as usize, (y_offset + (y as i32)) as usize, self.current_value_colour);
					}
				}
			}
		}

		#[cfg(test)] {
			debug!("Rendered candles @ {:?}", start_time.elapsed());
		}

		// Current, lowest, highest value line is rendered inside here.
		for (val, colour) in vec![(ohlc_of_set.l, self.down_colour), (ohlc_of_set.h, self.up_colour), (ohlc_of_set.c, self.current_value_colour)] {
			let y = height - (((val - ohlc_of_set.l) / y_val_increment).round() as usize) - margin_bottom;
			for half_x in (margin_left / 2)..((width - margin_right) / 2) {
				colour_rgba(&mut image_buffer, width, half_x * 2, y, colour);
			}

			// Add label
			{
				let mut chars = format!("{}{:.8}{}", self.value_prefix, val, self.value_suffix).into_bytes();

				while chars.len() > ((margin_right as f32 - 10.) / 10.).floor() as usize {
					let _ = chars.pop();
				}
				text_renders.push((chars, width - margin_right + 10, y - 8, colour, true))
			}
		}

		#[cfg(test)] {
			debug!("Rendered basic indicator lines @ {:?}", start_time.elapsed());
		}

		// Add title text
		text_renders.push((self.title.clone().into_bytes(), 8, 8, self.title_colour, false));

		// Text renderer section
		for (chars, base_x, base_y, colour, do_border) in text_renders {
			let chars_len = chars.len();

			if do_border {
				for x in (base_x - 1)..(base_x + 10 * chars_len as usize + 1) {
					for y_mag in 0..2 {
						let y = base_y + y_mag * 17 + y_mag * 1 - 1;

						colour_rgba(&mut image_buffer, width, x, y, colour);
					}
				}
				for x_mag in 0..2 {
					let x = base_x + x_mag * 10 * chars_len as usize + x_mag * 2 - 1;
					for y in (base_y - 1)..(base_y + 17 + 1) {
						colour_rgba(&mut image_buffer, width, x, y, colour);
					}
				}
			}

			// 10 is character width; f_x is starting at the left edge of the margin
			for f_x in 0usize..chars_len {
				let char_font: &[u8; 170] = &fonts::ASCII_TABLE[chars[(|d| if d < 127 { d } else { 0x20 })(f_x)] as usize];
				for incr_y in 0usize..17 {
					for incr_x in 0usize..10 {
						let x = base_x + (incr_x + f_x * 10) as usize;
						let y = base_y + incr_y as usize;

						let shade_at_pos = char_font[incr_x + incr_y * 10] as usize;

						if shade_at_pos == 0 {
							colour_rgba(&mut image_buffer, width, x, y, self.background_colour);
							continue;
						}

						// Don't modify the alpha channel
						for j in 0..3 {
							let curr_col = (colour >> (24 - 8 * j)) as u8;
							let bgc = (self.background_colour >> (24 - 8 * j)) as u8;

							image_buffer[(x + y * width) * 3 + j] = (
								((shade_at_pos * curr_col as usize +
									// Add the existing background instead of doing alphas
									((0xff - shade_at_pos) * bgc as usize)
								) as f64
									/ 255.
								).round()) as u8;
						}
					}
				}
			}
		}

		#[cfg(test)] {
			debug!("Rendered all text @ {:?}", start_time.elapsed());
		}

		{
			let mut ch_buffer = ChartBuffer::new(width, height, Margin {
				top: margin_top,
				bottom: margin_bottom,
				left: margin_left,
				right: margin_right,
			}, ohlc_of_set.h, ohlc_of_set.l, (self.time_units * data.len() as u64) as i64, &mut image_buffer[..]);

			for ext in &self.render_extensions {
				ext.apply(&self, &mut ch_buffer, &data[..]);

				#[cfg(test)] {
					debug!("Rendered extension:{} @ {:?}", ext.name(), start_time.elapsed());
				}
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

/// Colours the pixel with colour supplied by the RGBA field, alpha channel data is ignored.
fn colour_rgba(buffer: &mut Vec<u8>, width: usize, x: usize, y: usize, rgba: u32) {
	for j in 0..3 {
		buffer[(x + y * width) * 3 + j] = (rgba >> (24 - 8 * j)) as u8;
	}
}
