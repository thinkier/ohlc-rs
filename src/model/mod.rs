use std::mem;

pub type Point = (usize, usize);

pub struct Margin {
	pub top: usize,
	pub bottom: usize,
	pub left: usize,
	pub right: usize,
}

pub struct ChartBuffer<'a> {
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
	/// Byte buffer of the actual image
	pub buffer: &'a mut [u8],
}

impl<'a> ChartBuffer<'a> {
	pub(crate) fn new(width: usize, height: usize, margin: Margin, max_price: f64, min_price: f64, timeframe: i64, buffer: &'a mut [u8]) -> ChartBuffer {
		if buffer.len() != width * height * 3 {
			panic!("incorrectly initialized chart buffer! size must be width * height * 3");
		}

		if max_price > min_price {
			panic!("max > min... wut?");
		}

		if timeframe <= 0 {
			panic!("timeframe must be > 0");
		}

		if margin.top + margin.bottom > height || margin.left + margin.right > width {
			panic!("margins cannot be bigger than the image itself")
		}

		ChartBuffer { width, height, margin, max_price, min_price, timeframe, buffer }
	}

	/// Returns: (x, y)
	pub fn data_to_coords(&self, price: f64, time: i64) -> Point {
		let x = {
			let prog = time as f64 / self.timeframe as f64;

			if prog >= 0. {
				self.width - self.margin.right
			} else if prog <= -self.timeframe as f64 {
				self.margin.left
			} else {
				self.margin.right + (prog * (self.width - (self.margin.right + self.margin.left)) as f64) as usize
			}
		};

		let y = {
			let prog = price / (self.max_price - self.min_price);

			if prog >= 1. {
				self.margin.top
			} else if prog <= 0. {
				self.height - self.margin.bottom
			} else {
				self.margin.top + (prog * (self.height - (self.margin.top + self.margin.bottom)) as f64) as usize
			}
		};

		(x, y)
	}

	pub fn put(&mut self, price: f64, time: i64, rgba: u32) {
		let (x, y) = self.data_to_coords(price, time);
		self.colour(x, y, rgba);
	}

	/// Render a quadrilateral by the 4 given points and the colour filling
	pub fn quad(&mut self, mut p1: Point, mut p2: Point, mut p3: Point, mut p4: Point, rgba: u32) {
		unimplemented!()
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
	pub fn line(&mut self, mut p1: Point, mut p2: Point, thickness: usize, rgba: u32) {
		let thickness = thickness / 2;

		if p1.0 > p2.0 {
			mem::swap(&mut p1.0, &mut p2.0);
		}
		if p1.1 > p2.1 {
			mem::swap(&mut p1.1, &mut p2.1);
		}

		let run = p2.0 - p1.0;
		let rise = p2.1 - p1.1;
		let gradient = rise as f64 / run as f64;

		let mut y = p1.1 as f64;
		for x in p1.0..(p2.0 + 1) {
			y += gradient;
			let y = y.round() as usize;

			self.colour(x, y, rgba);
			if thickness >= 1 {
				self.colour(x, y + thickness, rgba);
				self.colour(x, y - thickness, rgba);
			}
		}
	}

	pub fn colour(&mut self, x: usize, y: usize, rgba: u32) {
		if x > self.width || y > self.height {
			return;
		}

		for j in 0..3 {
			self.buffer[(x + y * self.width) * 3 + j] = (rgba >> (24 - 8 * j)) as u8;
		}
	}
}
