use fonts::ASCII_TABLE;
pub use OHLCRenderOptions;
use std::mem;

pub type Point = (usize, usize);

pub trait Painter {
	fn buffer<'a>(&'a mut self) -> &'a mut Vec<u8>;

	fn width(&self) -> usize;

	fn height(&self) -> usize;

	fn background(&self) -> u32;

	/// Render a rectangle by the diagonally opposite points and colour
	fn rect_point(&mut self, p1: Point, p2: Point, rgba: u32) {
		self.rect(p1.0, p1.1, p2.0, p2.1, rgba);
	}

	/// Render a rectangle by the min/max x and y points and colour
	fn rect(&mut self, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize, rgba: u32) {
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

	/// Draw a line between two points
	fn line(&mut self, mut p1: Point, mut p2: Point, rgba: u32) {
		let mut pixels = vec![];

		if p1.0 > p2.0 {
			mem::swap(&mut p1, &mut p2);
		}

		let adjacent = (p2.0 as i64 - p1.0 as i64) as f64;
		let opposite = (p2.1 as i64 - p1.1 as i64) as f64;
		let tan = opposite / adjacent;

		for x in p1.0..p2.0 {
			let y = (p1.1 as f64 + if (x - p1.0) != 0 { tan * (x - p1.0) as f64 } else { 0. }) as usize;
			pixels.push((x, y));
		}

		if p1.1 > p2.1 {
			mem::swap(&mut p1, &mut p2);
		}

		for y in p1.1..p2.1 {
			let x = (p1.0 as f64 + if tan != 0. { (y - p1.1) as f64 / tan } else { 0. }) as usize;
			pixels.push((x, y));
		}

		pixels.dedup_by(|a, b| a == b);

		for (x, y) in pixels {
			self.colour(x, y, rgba);
		}
	}

	/// Colour a pixel by x and y coordinates
	fn colour(&mut self, x: usize, y: usize, rgba: u32) {
		let height = self.height();
		let width = self.width();
		let buffer = self.buffer();

		if x >= width || y >= height {
			return;
		}

		// Weird casts because I wanna strip the first 24 bits
		let alpha = (rgba as u8) as f64 / 255.;

		for j in 0..3 {
			let i = (x + y * width) * 3 + j;

			let applied_colour = {
				let colour = (rgba >> (24 - 8 * j)) as u8;
				if alpha >= 0.96 { // Lazy if opacity is >= 96%
					colour
				} else if alpha <= 0.04 { // Lazy if opacity is <= 4%
					continue;
				} else {
					let bgc = buffer[i];

					(((alpha * colour as f64) + ((1. - alpha) * bgc as f64)).round()) as u8
				}
			};

			buffer[i] = applied_colour;
		}
	}

	/// Colour a pixel located at point
	fn colour_point(&mut self, p: Point, rgba: u32) {
		self.colour(p.0, p.1, rgba);
	}

	/// Paint some text in the colour provided, starting in the top left corner specified
	fn text(&mut self, mut topleft: Point, text: &str, rgba: u32) {
		let bytes = text.as_bytes();
		for i in 0..bytes.len() {
			let byte = bytes[i];

			if byte == b'\n' {
				topleft.1 += 17;
				continue;
			}

			let table_idx = if byte > 127 { 0x20 } else { byte } as usize;

			let font_face = ASCII_TABLE[table_idx];
			for delta_x in 0..10 {
				for delta_y in 0..17 {
					let a = (((rgba as u8) as f64 / 255.) * font_face[delta_x + delta_y * 10] as f64) as u32;
					// let a = font_face[delta_x + delta_y * 10] as u32;
					self.colour(10 * i + topleft.0 + delta_x, topleft.1 + delta_y, ((rgba >> 8) << 8) + a);
				}
			}
		}
	}

	/// Draw text according to specifications and a box around it as well (give 1 pix of both x and y margin). Supports a single line only.
	fn text_with_outline(&mut self, topleft: Point, text: &str, rgba: u32) {
		let count = text.as_bytes().len();
		for delta_x in 0..count * 10 + 2 {
			let x = topleft.0 + delta_x;
			for delta_y in 0..19 {
				let y = topleft.1 + delta_y;

				let colour = if delta_y == 0 || delta_x == 0 || delta_x == count * 10 + 1 || delta_y == 18 { rgba } else { self.background() };
				self.colour(x, y, colour);
			}
		}

		self.text((topleft.0 + 1, topleft.1 + 1), text, rgba);
	}

	/// Draw text according to specifications and a background behind it as well. Supports a single line only.
	fn text_with_background(&mut self, topleft: Point, text: &str, rgba: u32, background_rgba: u32) {
		let count = text.as_bytes().len();
		for delta_x in 0..count * 10 {
			let x = topleft.0 + delta_x;
			for delta_y in 0..17 {
				let y = topleft.1 + delta_y;

				self.colour(x, y, background_rgba);
			}
		}

		self.text((topleft.0, topleft.1), text, rgba);
	}

	/// Paint the buffer in a certain colour
	fn colour_buffer(buffer: &mut Vec<u8>, area: usize, rgba: u32) {
		let r = (rgba >> 24) as u8;
		let g = (rgba >> 16) as u8;
		let b = (rgba >> 8) as u8;

		let colours = [r, g, b];

		for aj in 0..area * 3 {
			buffer.push(colours[aj % 3]);
		}
	}
}
