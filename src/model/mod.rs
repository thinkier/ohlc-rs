pub use data::OHLC;
use fonts::ASCII_TABLE;
pub use OHLCRenderOptions;
pub use self::basic_indicative_lines::BasicIndicativeLines;
pub use self::bollinger_bands::BollingerBands;
pub use self::grid_lines::GridLines;
pub use self::no_extension::NoExtension;
pub use self::ohlc_candles::OHLCCandles;
pub use self::rsi::RSI;
use std::fmt::Debug;
use std::mem;

pub mod basic_indicative_lines;
pub mod bollinger_bands;
pub mod grid_lines;
pub mod no_extension;
pub mod ohlc_candles;
pub mod rsi;
#[cfg(test)]
pub mod test_fill;
#[cfg(test)]
pub mod test_line;
#[cfg(test)]
pub mod test_text;


pub trait RendererExtension: Debug {
	fn apply(&self, _buffer: &mut ChartBuffer, _data: &[OHLC]);

	fn name(&self) -> String;
}

pub type Point = (usize, usize);

pub struct Margin {
	pub top: usize,
	pub bottom: usize,
	pub left: usize,
	pub right: usize,
}

pub struct ChartBuffer {
	/// Total width for the graph - this will be checked against the buffer
	width: usize,
	/// Total height for the graph - this will be checked against the buffer
	height: usize,
	/// Margin for the actual graph
	margin: Margin,
	/// Maximum price the graph is able to display
	max_price: f64,
	/// Minimum price the graph is able to display
	min_price: f64,
	/// The amount of time the graph covers, in seconds
	timeframe: i64,
	/// Default background colour, alpha channel is ignored
	background: u32,
	/// Byte buffer of the actual image
	pub buffer: Vec<u8>,
}

impl ChartBuffer {
	pub(crate) fn new(width: usize, height: usize, margin: Margin, max_price: f64, min_price: f64, timeframe: i64, background: u32) -> ChartBuffer {
		if max_price < min_price {
			panic!("max < min... wut?");
		}

		if timeframe <= 0 {
			panic!("timeframe must be > 0");
		}

		if margin.top + margin.bottom > height || margin.left + margin.right > width {
			panic!("margins cannot be bigger than the image itself")
		}

		let mut buffer = Vec::with_capacity(width * height * 3);

		{
			let r = (background >> 24) as u8;
			let g = (background >> 16) as u8;
			let b = (background >> 8) as u8;

			let colours = [r, g, b];

			for xyj in 0..width * height * 3 {
				buffer.push(colours[xyj % 3]);
			}
		}

		ChartBuffer { width, height, margin, max_price, min_price, timeframe, background: background | 0xFF, buffer }
	}

	/// Returns: (x, y)
	pub fn data_to_coords(&self, price: f64, time: i64) -> Point {
		let x = {
			let prog = time as f64 / self.timeframe as f64;

			if prog <= 0. {
				self.margin.left
			} else if prog >= 1. {
				self.width - self.margin.right
			} else {
				self.margin.left + (prog * (self.width - (self.margin.right + self.margin.left)) as f64) as usize
			}
		};

		let y = {
			let prog = (price - self.min_price) / (self.max_price - self.min_price);

			if prog >= 1. {
				self.margin.top
			} else {
				let bottom = self.height - self.margin.bottom;

				if prog <= 0. {
					bottom
				} else {
					(bottom as f64 - (prog * (bottom - self.margin.top) as f64)) as usize
				}
			}
		};

		(x, y)
	}

	pub fn put(&mut self, price: f64, time: i64, rgba: u32) {
		let (x, y) = self.data_to_coords(price, time);
		self.colour(x, y, rgba);
	}

	/// Render a rectangle by the diagonally opposite points and colour
	pub fn rect_point(&mut self, p1: Point, p2: Point, rgba: u32) {
		self.rect(p1.0, p1.1, p2.0, p2.1, rgba);
	}

	/// Render a rectangle by the min/max x and y points and colour
	pub fn rect(&mut self, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize, rgba: u32) {
		if x1 > x2 {
			mem::swap(&mut x1, &mut x2);
		}
		if y1 > y2 {
			mem::swap(&mut y1, &mut y2);
		}

		for x in x1..(x2 + 1) {
			for y in y1..(y2 + 1) {
				self.colour(x, y, rgba);
			}
		}
	}

	/// Render a line between 2 points
	pub fn line(&mut self, mut p1: Point, mut p2: Point, rgba: u32) {
		if p1.0 > p2.0 {
			mem::swap(&mut p1, &mut p2);
		}

		let adjacent = Self::distance(p1.0 as isize, p2.0 as isize) as f64;
		let opposite = Self::distance(p1.1 as isize, p2.1 as isize) as f64;
		let tan = opposite / adjacent;

		for x in p1.0..p2.0 {
			let y = (p1.1 as f64 + if (x - p1.0) != 0 { tan * (x - p1.0) as f64 } else { 0. }) as usize;
			self.colour(x, y, rgba);
		}

		if p1.1 > p2.1 {
			mem::swap(&mut p1, &mut p2);
		}

		for y in p1.1..p2.1 {
			let x = (p1.0 as f64 + if tan != 0. { (y - p1.1) as f64 / tan } else { 0. }) as usize;
			self.colour(x, y, rgba);
		}
	}

	pub fn colour_point(&mut self, point: Point, rgba: u32) {
		self.colour(point.0, point.1, rgba);
	}

	pub fn colour(&mut self, x: usize, y: usize, rgba: u32) {
		if x >= self.width || y >= self.height {
			return;
		}

		// Weird casts because I wanna strip the first 24 bits
		let alpha = (rgba as u8) as f64 / 255.;

		for j in 0..3 {
			let i = (x + y * self.width) * 3 + j;

			let applied_colour = {
				let colour = (rgba >> (24 - 8 * j)) as u8;
				if alpha >= 0.96 { // Lazy if opacity is >= 96%
					colour
				} else if alpha <= 0.04 { // Lazy if opacity is <= 4%
					continue;
				} else {
					let bgc = self.buffer[i];

					(((alpha * colour as f64) + ((1. - alpha) * bgc as f64)).round()) as u8
				}
			};

			self.buffer[i] = applied_colour;
		}
	}

	pub fn text(&mut self, min: Point, text: &str, rgba: u32) {
		let bytes = text.as_bytes();
		for i in 0..bytes.len() {
			let font_face = ASCII_TABLE[(|b| { if b > 127 { 0x20 } else { b } })(bytes[i]) as usize];
			for delta_x in 0..10 {
				for delta_y in 0..17 {
					let a = (((rgba as u8) as f64 / 255.) * font_face[delta_x + delta_y * 10] as f64) as u32;
					// let a = font_face[delta_x + delta_y * 10] as u32;
					self.colour(10 * i + min.0 + delta_x, min.1 + delta_y, ((rgba >> 8) << 8) + a);
				}
			}
		}
	}

	pub fn text_with_outline(&mut self, min: Point, text: &str, rgba: u32) {
		let count = text.as_bytes().len();
		for delta_x in 0..count * 10 + 2 {
			let x = min.0 + delta_x;
			for delta_y in 0..19 {
				let y = min.1 + delta_y;

				let colour = if delta_y == 0 || delta_x == 0 || delta_x == count * 10 + 1 || delta_y == 18 { rgba } else { self.background };
				self.colour(x, y, colour);
			}
		}

		self.text((min.0 + 1, min.1 + 1), text, rgba);
	}

	pub fn distance(x1: isize, x2: isize) -> isize {
		x2 - x1
	}
}
